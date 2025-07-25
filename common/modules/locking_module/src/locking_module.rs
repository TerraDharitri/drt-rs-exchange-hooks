dharitri_sc::imports!();

mod simple_lock {
    dharitri_sc::imports!();
    dharitri_sc::derive_imports!();

    #[dharitri_sc::proxy]
    pub trait SimpleLockProxy {
        #[payable("*")]
        #[endpoint(lockTokens)]
        fn lock_tokens_endpoint(
            &self,
            unlock_epoch: u64,
            opt_destination: OptionalValue<ManagedAddress>,
        ) -> RewaOrDcdtTokenPayment;
    }
}

#[dharitri_sc::module]
pub trait LockingModule {
    #[only_owner]
    #[endpoint(setLockingScAddress)]
    fn set_locking_sc_address(&self, new_address: ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(&new_address),
            "Invalid SC Address"
        );

        self.locking_sc_address().set(&new_address);
    }

    #[only_owner]
    #[endpoint(setUnlockEpoch)]
    fn set_unlock_epoch(&self, new_epoch: u64) {
        self.unlock_epoch().set(new_epoch);
    }

    #[inline]
    fn lock_tokens(
        &self,
        token_id: RewaOrDcdtTokenIdentifier,
        amount: BigUint,
    ) -> RewaOrDcdtTokenPayment<Self::Api> {
        self.lock_common(OptionalValue::None, token_id, amount)
    }

    #[inline]
    fn lock_tokens_and_forward(
        &self,
        to: ManagedAddress,
        token_id: RewaOrDcdtTokenIdentifier,
        amount: BigUint,
    ) -> RewaOrDcdtTokenPayment<Self::Api> {
        self.lock_common(OptionalValue::Some(to), token_id, amount)
    }

    fn lock_common(
        &self,
        opt_dest: OptionalValue<ManagedAddress>,
        token_id: RewaOrDcdtTokenIdentifier,
        amount: BigUint,
    ) -> RewaOrDcdtTokenPayment<Self::Api> {
        let unlock_epoch = self.unlock_epoch().get();
        let mut proxy_instance = self.get_locking_sc_proxy_instance();

        proxy_instance
            .lock_tokens_endpoint(unlock_epoch, opt_dest)
            .with_rewa_or_single_dcdt_transfer((token_id, 0, amount))
            .execute_on_dest_context()
    }

    fn get_locking_sc_proxy_instance(&self) -> simple_lock::ProxyTo<Self::Api> {
        let locking_sc_address = self.locking_sc_address().get();
        self.locking_sc_proxy_obj(locking_sc_address)
    }

    #[proxy]
    fn locking_sc_proxy_obj(&self, sc_address: ManagedAddress) -> simple_lock::ProxyTo<Self::Api>;

    #[view(getLockingScAddress)]
    #[storage_mapper("lockingScAddress")]
    fn locking_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getUnlockEpoch)]
    #[storage_mapper("unlockEpoch")]
    fn unlock_epoch(&self) -> SingleValueMapper<u64>;
}
