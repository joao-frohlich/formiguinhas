// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use formiguinhas::ant::*;
use formiguinhas::board::*;

fn main() {
    let width = 50;
    let height = 50;
    let dead_ants = 1000;
    let agents = 10;
    let max_iter = 100000;
    let radius = 1;
    let threshold = 0.45;
    let min_prob = 0.00000;
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Formiguinhas".to_string(),
            width: 600.,
            height: 600.,
            resizable: false,
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<Board>()
        .insert_resource(Board::new(
            width, height, dead_ants, agents, max_iter, radius, threshold, min_prob,
        ))
        .add_startup_system_to_stage(StartupStage::PreStartup, setup_board)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_dead_ants)
        .add_startup_system(setup_agents)
        .add_system(color_cells)
        .add_system(draw_agents)
        .add_system(move_agent)
        .add_system(set_visibility)
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
