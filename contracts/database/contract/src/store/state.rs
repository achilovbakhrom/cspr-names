use common_lib::{
    db::store::Store
};
use common_lib::constants::{KEY_DATABASE_TOTALS_DOMAIN_COUNT, KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT};
use common_lib::db::traits::Storable;

pub(crate) struct TotalState {
    store: Store
}

impl TotalState {

    pub fn instance() -> Self {
        Self {
            store: Store::instance()
        }
    }

    pub fn increment_domains_count(&self) {
        self.increment_domains_count_by(1);
    }

    pub fn increment_domains_count_by(&self, by: u64) {
        let mut count = self.store.get::<u64>(KEY_DATABASE_TOTALS_DOMAIN_COUNT).unwrap_or(0);
        count += by;
        self.store.set(KEY_DATABASE_TOTALS_DOMAIN_COUNT, count);
    }

    pub fn decrement_domains_count(&self) {
        self.decrement_domains_count_by(1);
    }

    pub fn decrement_domains_count_by(&self, by: u64) {
        let mut count = self.store.get::<u64>(KEY_DATABASE_TOTALS_DOMAIN_COUNT).unwrap_or(0);
        if count != 0 {
            count -= by;
        }
        self.store.set(KEY_DATABASE_TOTALS_DOMAIN_COUNT, count);
    }

    pub fn increment_subdomains_count(&self) {
        self.increment_subdomains_count_by(1);
    }

    pub fn increment_subdomains_count_by(&self, by: u64) {
        let mut count = self.store.get::<u64>(KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT).unwrap_or(0);
        count += by;
        self.store.set(KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT, count);
    }

    pub fn decrement_subdomains_count(&self) {
        self.decrement_subdomains_count_by(1)
    }

    pub fn decrement_subdomains_count_by(&self, by: u64) {
        let mut count = self.store.get::<u64>(KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT).unwrap_or(0);
        if count != 0 {
            count -= by;
        }
        self.store.set(KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT, count);
    }

    pub fn get_totals(&self) -> (u64, u64) {
        let domains_count = self.store.get::<u64>(KEY_DATABASE_TOTALS_DOMAIN_COUNT).unwrap_or(0);
        let subdomains_count = self.store.get::<u64>(KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT).unwrap_or(0);
        (domains_count, subdomains_count)
    }

}