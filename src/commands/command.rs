use regex::Regex;
use anyhow::{Result, bail};

use crate::commands::claim;

// E.g., Claim ENS name for address 0xafBD210c60dD651892a61804A989eEF7bD63CBA0 with resolver resolver.eth 
pub fn exec(verification_key: &str, proof: &str, inp: &str, dkim_pk: &str, email: &str, command: &str) -> Result<()> {
    let pattern = r"Claim ENS name for address (?P<address>0x[a-fA-F0-9]{40}) with resolver (?P<resolver>[^\s]+)";
    let re = Regex::new(pattern)?;
    if let Some(caps) = re.captures(command) {
        let address = &caps["address"];
        let resolver = &caps["resolver"];
        claim::exec(verification_key, proof, inp, dkim_pk, address, resolver, email)
    } else {
       bail!("Invalid command")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_claim_proof_verification_ok() -> Result<()> {
        exec("fixtures/vkey.json",
            "fixtures/proof.json",
            "fixtures/public.json",
            "0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            "thezdev1@gmail.com",
            "Claim ENS name for address 0xafBD210c60dD651892a61804A989eEF7bD63CBA0 with resolver resolver.eth"
        )
    }

    #[test]
    #[should_panic]
    fn test_claim_proof_verification_bad_command() {
        exec("fixtures/vkey.json",
            "fixtures/proof.json",
            "fixtures/public.json",
            "0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            "thezdev1@gmail.com",
            "Some wrong command"
        ).unwrap()
    }

    #[test]
    #[should_panic]
    fn test_claim_proof_verification_wrong_resolver() {
        exec("fixtures/vkey.json",
            "fixtures/proof.json",
            "fixtures/public.json",
            "0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            "thezdev1@gmail.com",
            "Claim ENS name for address 0xafBD210c60dD651892a61804A989eEF7bD63CBA0 with resolver resolver2.eth"
        ).unwrap()
    }

    #[test]
    #[should_panic]
    fn test_claim_proof_verification_wrong_addr() {
        exec("fixtures/vkey.json",
            "fixtures/proof.json",
            "fixtures/public.json",
            "0ea9c777dc7110e5a9e89b13f0cfc540e3845ba120b2b6dc24024d61488d4788",
            "thezdev1@gmail.com",
            "Claim ENS name for address 0xafBD210c60dD651892a61804A989eEF7bD63CBA1 with resolver resolver.eth"
        ).unwrap()
    }

}
