use eth2_types::eth_spec::EthSpec;
pub use simulation::{args, Error as SimulationError, Simulation};
use snafu::{OptionExt, ResultExt, Snafu};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use types as eth2_types;

/// Shorthand for result types returned from Dispatch.
pub type Result<V, E = Error> = std::result::Result<V, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    /// Simulation error
    // Called "Sim" instead of "Simulation" to prevent name collision because Snafu auto-generates
    // context selectors with the same name as the enum values
    Sim {
        source: SimulationError,
    },
    /// Operation was cancelled because the simulation is shutting down.
    Terminated,
}

#[derive(Debug)]
enum Operation<T>
where
    T: EthSpec,
{
    CreateExecutionEnvironment(args::CreateExecutionEnvironment, Sender<Result<eth2_types::slot_epoch_root::EeIndex>>),
    CreateShardBlock(args::CreateShardBlock, Sender<Result<eth2_types::slot_epoch_root::ShardSlot>>),
    GetExecutionEnvironment(
        args::GetExecutionEnvironment,
        Sender<Result<eth2_types::execution_environment::ExecutionEnvironment<T>>>,
    ),
    GetExecutionEnvironmentState(args::GetExecutionEnvironmentState, Sender<Result<eth2_types::slot_epoch_root::Root>>),
    GetShardBlock(args::GetShardBlock, Sender<Result<eth2_types::shard_block::ShardBlock>>),
    GetShardState(args::GetShardState, Sender<Result<eth2_types::shard_state::ShardState<T>>>),
}

#[derive(Debug)]
pub struct Dispatch<T>
where T: EthSpec,
{
    simulation: Simulation<T>,
    receiver: Receiver<Operation<T>>,
}

impl<T: EthSpec> Dispatch<T> {
    pub fn new(simulation: Simulation<T>) -> (Self, Handle<T>) {
        let (sender, receiver) = channel(1);
        let handle = Handle {
            sender
        };

        let me: Dispatch<T> = Dispatch {
            simulation,
            receiver,
        };

        (me, handle)
    }

    pub async fn run(mut self) -> Result<()> {
        eprintln!("Simulation Running: {:?}", std::thread::current().id());
        while let Some(op) = self.receiver.recv().await {
            match op {
                Operation::CreateExecutionEnvironment(args, mut reply) => {
                    let res = self.simulation.create_execution_environment(args).context(Sim);
                    reply.send(res).await;
                }
                Operation::CreateShardBlock(args, mut reply) => {
                    let res = self.simulation.create_shard_block(args).context(Sim);
                    reply.send(res).await;
                }
                Operation::GetExecutionEnvironment(args, mut reply) => {
                    let res = self.simulation.get_execution_environment(args).context(Sim);
                    reply.send(res).await;
                }
                Operation::GetExecutionEnvironmentState(args, mut reply) => {
                    let res = self.simulation.get_execution_environment_state(args).context(Sim);
                    reply.send(res).await;
                }
                Operation::GetShardBlock(args, mut reply) => {
                    let res = self.simulation.get_shard_block(args).context(Sim);
                    reply.send(res).await;
                }
                Operation::GetShardState(args, mut reply) => {
                    let res = self.simulation.get_shard_state(args).context(Sim);
                    reply.send(res).await;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Handle<T>
where
    T: EthSpec
{
    sender: Sender<Operation<T>>,
}

impl<T: EthSpec> Handle<T> {
    pub async fn create_execution_environment(
        &mut self,
        arg: args::CreateExecutionEnvironment,
    ) -> Result<eth2_types::slot_epoch_root::EeIndex> {
        let (sender, mut receiver) = channel(1);

        self.sender
            .send(Operation::CreateExecutionEnvironment(arg, sender))
            .await;

        receiver.recv().await.context(Terminated)?
    }

    pub async fn create_shard_block(&mut self, arg: args::CreateShardBlock) -> Result<eth2_types::slot_epoch_root::ShardSlot> {
        let (sender, mut receiver) = channel(1);

        self.sender.send(Operation::CreateShardBlock(arg, sender)).await;

        receiver.recv().await.context(Terminated)?
    }

    pub async fn get_execution_environment(&mut self, arg: args::GetExecutionEnvironment) -> Result<eth2_types::execution_environment::ExecutionEnvironment<T>> {
        let (sender, mut receiver) = channel(1);

        self.sender.send(Operation::GetExecutionEnvironment(arg, sender)).await;

        receiver.recv().await.context(Terminated)?
    }

    pub async fn get_execution_environment_state(&mut self, arg: args::GetExecutionEnvironmentState) -> Result<eth2_types::slot_epoch_root::Root> {
        let (sender, mut receiver) = channel(1);

        self.sender.send(Operation::GetExecutionEnvironmentState(arg, sender)).await;

        receiver.recv().await.context(Terminated)?
    }

    pub async fn get_shard_block(&mut self, arg: args::GetShardBlock) -> Result<eth2_types::shard_block::ShardBlock> {
        let (sender, mut receiver) = channel(1);

        self.sender.send(Operation::GetShardBlock(arg, sender)).await;

        receiver.recv().await.context(Terminated)?
    }

    pub async fn get_shard_state(&mut self, arg: args::GetShardState) -> Result<eth2_types::shard_state::ShardState<T>> {
        let (sender, mut receiver) = channel(1);

        self.sender.send(Operation::GetShardState(arg, sender)).await;

        receiver.recv().await.context(Terminated)?
    }
}