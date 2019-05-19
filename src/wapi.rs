use util::*;
use model::*;
use client::*;
use errors::*;
use std::collections::BTreeMap;
use serde_json::from_str;

#[derive(Clone)]
pub struct Wapi {
    pub client: Client,
    pub recv_window: u64,
}

impl Wapi {
    // Account Information
    pub fn get_deposit_history(&self) -> Result<(DepositHistory)> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        let data = self.client.get_signed("/wapi/v3/depositHistory.html", &request)?;
        let deposit_history: DepositHistory = from_str(data.as_str())?;

        Ok(deposit_history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const API_KEY: &str = "1FiZGJq8IXdwk4atORw1bX0OFXs8kFfwnAKy41kb9IL23m1OYSJD6NbTmq8wjXAH";
    const API_SECRET: &str = "Cpck8TmUK99IB5CGOuiVyGH9OjVwd5fywOpqdpKXOMjhu1ffrHFGnTM7EQb2QWoj";

    #[test]
    fn test_get_deposit() -> Result<()> {
        let wapi = Wapi {
            client: Client::new(
                Some(API_KEY.to_string()),
                Some(API_SECRET.to_string())
            ),
            recv_window: 5000,
        };
        let history = wapi.get_deposit_history()?;
        println!("{:?}", history);
        assert!(history.deposit_list.len() > 0);
        Ok(())
    }
}