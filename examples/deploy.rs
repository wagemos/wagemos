use abstract_app::abstract_interface::{AppDeployer, DeployStrategy};
use abstract_client::{AbstractClient, AccountBuilder, Publisher, PublisherBuilder};
use abstract_core::objects::gov_type::GovernanceDetails;
use abstract_core::objects::namespace::Namespace;
use abstract_core::registry::ExecuteMsgFns;
use abstract_interface::{Abstract, AccountDetails, AccountI};
use clap::Parser;
use abstract_betting_app::{contract::interface::Bet, BET_APP_ID};
use cw_orch::{
    anyhow,
    prelude::{networks::parse_network, DaemonBuilder},
    tokio::runtime::Runtime,
};
use cw_orch::daemon::{Daemon, TxSender};
use cw_orch::environment::ChainInfo;
use cw_orch::prelude::ContractInstance;
use semver::Version;
use abstract_betting_app::contract::{BetApp, CONTRACT_VERSION};
use abstract_betting_app::msg::BetInstantiateMsg;

fn deploy(networks: Vec<ChainInfo>) -> anyhow::Result<()> {
    let version: Version = CONTRACT_VERSION.parse().unwrap();

    // run for each requested network
    for network in networks {
        let rt = Runtime::new()?;
        let chain = DaemonBuilder::new(network).handle(rt.handle()).build()?;

        let abstr = AbstractClient::new(chain.clone())?;
        let abs = Abstract::new(chain.clone());

        let acc = AccountI::create(&abs, AccountDetails {
            name: "".to_string(),
            description: None,
            link: None,
            namespace: Some("wagemos".to_string()),
            install_modules: vec![],
            account_id: None,
        }, GovernanceDetails::Monarchy {
            monarch: chain.sender().address().to_string()
        }, &[])?;

        panic!("Account: {:?}", acc.id());
        // Caused by:
        //     Error parsing into type u32: EOF while parsing a JSON value.
        let wagemos_acc = abstr.account_builder().name("Wagemos test").build()?;


        panic!("Wagemos account: {:?}", wagemos_acc.id());

        abstr.registry().claim_namespace(wagemos_acc.id().unwrap(), "wagemos")?;


        // let publisher = abstr.publisher_builder(Namespace::from_id(BET_APP_ID).unwrap()).build()?;

        let bet = Bet::new(BET_APP_ID, chain.clone());

        // publisher.publish_app::<Bet<Daemon>>()?;
        bet.deploy(version.clone(), DeployStrategy::Force)?;

        let acc = abstr.account_builder().namespace(Namespace::new("wagemos-test")?).install_on_sub_account(false).build()?;
        let installed = acc.install_app::<Bet<Daemon>>(&BetInstantiateMsg {
            rake: None
        }, &[])?;

        println!("Installed: {:?}", installed.address()?);

    }

    Ok(())
}

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network Id to deploy on
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    network_ids: Vec<String>,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let args = Arguments::parse();
    let networks = args.network_ids.iter().map(|n| parse_network(n).unwrap()).collect();
    deploy(networks).unwrap();
}
