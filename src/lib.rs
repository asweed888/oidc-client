// start auto exported by saba.
pub mod act;
// end auto exported by saba.

pub mod prelude;

use act::google::GoogleOidcClientAct;

pub struct GoogleOidcClient;

impl GoogleOidcClient {
    pub fn new() -> Self {
        Self{}
    }
}

impl GoogleOidcClientAct for GoogleOidcClient {}
