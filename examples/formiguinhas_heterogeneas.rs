// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use formiguinhas::heterogeneous_ant::*;
use formiguinhas::heterogeneous_board::*;
use formiguinhas::params::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut k1: f32 = 0.3;
    let mut k2: f32 = 0.4;
    if args.len() == 3 {
        let query = &args[1];
        k1 = query.parse().expect("k1 should be float");
        let query = &args[2];
        k2 = query.parse().expect("k2 should be float");
    }
    let width = 50;
    let height = 50;
    let max_iter = 1000000;
    // let iter_per_render = 10000;
    let dead_ants = 1000;
    let agents = 10;
    let radius = 1;
    let threshold = 0.45;
    let min_prob = 0.00000;
    let items = 400;
    let base_path = "bases/base4.txt".to_string();
    let colors = [(0.75, 0.25, 0.25),(0.25, 0.25, 0.75),(0.75, 0.25, 0.75),(0.75, 0.75, 0.25)].to_vec();
    let alpha = 30.0;
    // let items = 600;
    // let base_path = "bases/base15.txt".to_string();
    // let colors = [
    //     (0.925, 0.486, 0.149),(0.243, 0.231, 0.196),(0.164, 0.392, 0.47),(0.56, 0.545, 0.4),
    //     (0.258, 0.274, 0.196),(0.78, 0.705, 0.274),(0.917, 0.902, 0.792),(0.509, 0.509, 0.509),
    //     (0.796, 0.157, 0.129),(0.631, 0.137, 0.07),(0.188, 0.517, 0.274),(0.117, 0.117, 0.117),
    //     (0.956, 0.662, 0.),(0., 0.956, 0.662),(0.47, 0.121, 0.098)].to_vec();

    //let width: usize = f32::powf(10.*items as f32, 1./2.) as usize;
    //let height: usize = f32::powf(10.*items as f32, 1./2.) as usize;
    //let max_iter = usize::max(1000000, 2000*items);
    //let iter_per_render: usize = f32::powf(20.*items as f32, 1./2.) as usize;
    let iter_per_render = max_iter;

    // println!("{} {} {} {}", width, height, max_iter, iter_per_render);

    let window_name = "Formiguinhas ".to_string() + k1.to_string().as_str() + " " + k2.to_string().as_str();

    App::new()
        .insert_resource(WindowDescriptor {
            title: window_name,
            width: 700.,
            height: 700.,
            resizable: false,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<HeterogeneousBoard>()
        .insert_resource(HeterogeneousBoard::new(width, height))
        .insert_resource(Params::new(
            dead_ants,
            agents,
            max_iter,
            iter_per_render,
            radius,
            threshold,
            min_prob,
            base_path,
            colors,
            k1,
            k2,
            alpha
        ))
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_board)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_items)
        .add_startup_system(setup_agents)
        .add_system(color_cells)
        .add_system(draw_agents)
        .add_system(move_agent)
        .add_system(set_visibility)
        .add_system(exit_app)
        .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/*
Descrição do problema:
Dado um grid NxM, onde cada célula representa um pedaço de uma colônia de formigas,
e pode conter exclusivamente uma formiga morta e exclusivamente uma formiga viva,
simular o comportamento das formigas vivas, onde elas devem agrupar as formigas
mortas.

Modelagem do problema:

Agentes: formigas vivas
Ambiente: grid NxM
Sensores dos agentes: visão que permite visualizar o que está contido em cada célula
dentro de uma área (livre, formiga morta, formiga viva), definida pelo raio da visão
Atuadores dos agentes: caminhar (aleatoriamente), pegar formiga morta (se não estiver
carregando uma), largar formiga morta (se estiver carregando uma)

Características do problema: parcialmente observável, ...

Parâmetros do problema:
grid: 50x50
formigas mortas: 1000
formigas vivas: 10

Passos:
1) Inicialmente, gerar um grid NxM, com X formigas mortas e Y formigas vivas espalhadas
de maneira uniforme pelo grid.

2) Simular a movimentação das formigas. A movimentação deve ser aleatória, podendo a
formiga andar para 1 das 4 direções (N, L, S, O) com probabilidade igual. Aqui,
adicionaremos uma restrição de que a formiga não pode ir diretamente (isto é, movimento
para N, movimento para S) para a célula de onde ela veio, nem pode ir para fora do grid.
Ex.1: Suponha que a formiga chegou à célula atual usando o movimento N, portanto, nessa
rodada ela não pode se movimentar para S. Caso
Ex.2: Suponha que a formiga esteja numa posição x,M, onde x > 1 e x < N, a formiga não
poderá se movimentar para L, pois assim ele estaria saindo do grid

3) Implementar as regras para pegar e largar um item. Antes, porém, de definir as regras,
algumas definições importantes:

3.1) O agente consegue apenas enxergar à frente, podendo enxergar R fileiras do grid à
sua frente, onde R é o raio da visão do agente. A cada fileira, o agente consegue enxergar
duas células a mais que a fileira anterior, como mostra a seguinte ilustração:
...........
.....#.....
....XXX....
...X#XXX...
..#X##XX#..
.#B##A#X##.
..#X#####..
...#XXB#...
....###....
.....#.....
...........
Aqui, R=4. Nessa ilustração, 'A' representa a posição do agente, '.' é uma célula que o
agente não consegue enxergar, enquanto que #, X e B representam células visíveis ao agente.
# representa uma célula vazia, X representa uma célula com uma formiga morta, e B
representa a existência de outro agente. O número total de células será definido pela
variável T, enquanto que o número de células ocupadas por formigas mortas (descontando a
posição do agente) será definido por S. Nesse exemplo, T = 40 e S = 14.

As regras, então, para pegar e largar um item, são:

3.2) Pegar um item: Se a formiga não estiver carregando uma formiga morta, e a posição
onde ela está agora estiver ocupada pela formiga morta, a decisão de pegar a formiga
será definida aleatóriamente pela seguinte função:
pegar(S,T) = 0.005+(S/T-0.01)

No mínimo 0,5% e no máximo 99,5% de probabilidade
S == T -> 99,5%
S == 0 -> 00,5%

O que eu quero favorecer?
Se tiver uma quantidade pequena de itens, quero uma chance menor
Se tiver uma quantidade grande de itens, quero uma chance maior

Quando parar a iteração, sumir com as formigas livres e deixar as formigas restantes
iterarem até largarem os itens


*/

/*
Implementação:




*/

//|
