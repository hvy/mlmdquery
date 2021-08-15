use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    /// Counts artifacts/executions/contexts/events.
    Count(CountOpt),

    /// Gets artifacts/executions/contexts/events.
    Get(GetOpt),

    /// Generates graphs in DOT language.
    Graph(GraphOpt),
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum CountOpt {
    /// Counts artifacts.
    Artifacts(mlmdquery::artifacts::CountArtifactsOpt),

    /// Counts artifact types.
    ArtifactTypes(mlmdquery::artifact_types::ArtifactTypesOpt),

    /// Counts executions.
    Executions(mlmdquery::executions::CountExecutionsOpt),

    /// Counts execution types.
    ExecutionTypes(mlmdquery::execution_types::ExecutionTypesOpt),

    /// Counts contexts.
    Contexts(mlmdquery::contexts::CountContextsOpt),

    /// Counts context types.
    ContextTypes(mlmdquery::context_types::ContextTypesOpt),

    /// Counts events.
    Events(mlmdquery::events::CountEventsOpt),
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum GetOpt {
    /// Gets artifacts.
    Artifacts(mlmdquery::artifacts::GetArtifactsOpt),

    /// Gets artifact types.
    ArtifactTypes(mlmdquery::artifact_types::ArtifactTypesOpt),

    /// Gets executions.
    Executions(mlmdquery::executions::GetExecutionsOpt),

    /// Gets execution types.
    ExecutionTypes(mlmdquery::execution_types::ExecutionTypesOpt),

    /// Gets contexts.
    Contexts(mlmdquery::contexts::GetContextsOpt),

    /// Gets context types.
    ContextTypes(mlmdquery::context_types::ContextTypesOpt),

    /// Gets events.
    Events(mlmdquery::events::GetEventsOpt),
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum GraphOpt {
    /// Generates a graph showing the lineage of an artifact.
    Lineage(mlmdquery::lineage::GraphLineageOpt),

    /// Generates a graph showing the input and output of an execution.
    Io(mlmdquery::io::GraphIoOpt),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Count(CountOpt::Artifacts(opt)) => print_json(opt.count().await?)?,
        Opt::Get(GetOpt::Artifacts(opt)) => print_json(opt.get().await?)?,
        Opt::Count(CountOpt::ArtifactTypes(opt)) => print_json(opt.count().await?)?,
        Opt::Get(GetOpt::ArtifactTypes(opt)) => print_json(opt.get().await?)?,
        Opt::Count(CountOpt::Executions(opt)) => print_json(opt.count().await?)?,
        Opt::Get(GetOpt::Executions(opt)) => print_json(opt.get().await?)?,
        Opt::Count(CountOpt::ExecutionTypes(opt)) => print_json(opt.count().await?)?,
        Opt::Get(GetOpt::ExecutionTypes(opt)) => print_json(opt.get().await?)?,
        Opt::Count(CountOpt::Contexts(opt)) => print_json(opt.count().await?)?,
        Opt::Get(GetOpt::Contexts(opt)) => print_json(opt.get().await?)?,
        Opt::Count(CountOpt::ContextTypes(opt)) => print_json(opt.count().await?)?,
        Opt::Get(GetOpt::ContextTypes(opt)) => print_json(opt.get().await?)?,
        Opt::Count(CountOpt::Events(opt)) => print_json(opt.count().await?)?,
        Opt::Get(GetOpt::Events(opt)) => print_json(opt.get().await?)?,
        Opt::Graph(GraphOpt::Lineage(opt)) => opt.graph(&mut std::io::stdout().lock()).await?,
        Opt::Graph(GraphOpt::Io(opt)) => opt.graph(&mut std::io::stdout().lock()).await?,
    }
    Ok(())
}

fn print_json(item: impl serde::Serialize) -> anyhow::Result<()> {
    serde_json::to_writer_pretty(std::io::stdout().lock(), &item)?;
    println!();
    Ok(())
}
