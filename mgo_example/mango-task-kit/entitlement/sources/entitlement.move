module s3_test_entitlement::entitlement {
    use std::vector;
    use mgo::dynamic_field;
    use mgo::event;
    use mgo::object::{Self, UID};
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};

    const EWrongIssuerRegistry: u64 = 0;
    const EInvalidTaskId: u64 = 1;
    const EInvalidPayloadDigest: u64 = 2;
    const EBindingMismatch: u64 = 3;

    struct Registry has key {
        id: UID,
        issuance_count: u64,
    }

    struct IssuerCap has key {
        id: UID,
        registry: address,
    }

    struct IssuanceBinding has store {
        business_payload_digest: vector<u8>,
        recipient: address,
        entitlement: address,
    }

    /// An address-owned record for the local nonfinancial test.
    struct TestEntitlement has key {
        id: UID,
        registry: address,
        task_id: vector<u8>,
        business_payload_digest: vector<u8>,
        recipient: address,
    }

    struct EntitlementIssued has copy, drop {
        registry: address,
        entitlement: address,
        task_id: vector<u8>,
        business_payload_digest: vector<u8>,
        recipient: address,
    }

    fun init(ctx: &mut TxContext) {
        let registry = Registry { id: object::new(ctx), issuance_count: 0 };
        let cap = IssuerCap { id: object::new(ctx), registry: object::id_address(&registry) };
        transfer::share_object(registry);
        transfer::transfer(cap, tx_context::sender(ctx));
    }

    public entry fun issue(
        registry: &mut Registry,
        issuer: &IssuerCap,
        task_id: vector<u8>,
        business_payload_digest: vector<u8>,
        recipient: address,
        ctx: &mut TxContext,
    ) {
        let registry_address = object::id_address(registry);
        assert!(issuer.registry == registry_address, EWrongIssuerRegistry);
        assert!(vector::length(&task_id) == 32, EInvalidTaskId);
        assert!(vector::length(&business_payload_digest) == 32, EInvalidPayloadDigest);

        if (dynamic_field::exists_with_type<vector<u8>, IssuanceBinding>(&registry.id, copy task_id)) {
            let existing = dynamic_field::borrow<vector<u8>, IssuanceBinding>(&registry.id, task_id);
            assert!(
                &existing.business_payload_digest == &business_payload_digest && existing.recipient == recipient,
                EBindingMismatch,
            );
            return
        };

        let entitlement = TestEntitlement {
            id: object::new(ctx),
            registry: registry_address,
            task_id: copy task_id,
            business_payload_digest: copy business_payload_digest,
            recipient,
        };
        let entitlement_address = object::id_address(&entitlement);
        dynamic_field::add(&mut registry.id, copy task_id, IssuanceBinding {
            business_payload_digest: copy business_payload_digest,
            recipient,
            entitlement: entitlement_address,
        });
        registry.issuance_count = registry.issuance_count + 1;
        transfer::transfer(entitlement, recipient);
        event::emit(EntitlementIssued {
            registry: registry_address,
            entitlement: entitlement_address,
            task_id,
            business_payload_digest,
            recipient,
        });
    }

    #[test_only]
    use mgo::test_scenario;

    #[test_only]
    fun check_replay(replay_digest: vector<u8>, replay_recipient: address) {
        let operator = @0x51;
        let scenario = test_scenario::begin(operator);
        init(test_scenario::ctx(&mut scenario));
        test_scenario::next_tx(&mut scenario, operator);

        let task = x"1111111111111111111111111111111111111111111111111111111111111111";
        let digest = x"2222222222222222222222222222222222222222222222222222222222222222";
        let registry = test_scenario::take_shared<Registry>(&scenario);
        let cap = test_scenario::take_from_sender<IssuerCap>(&scenario);
        issue(&mut registry, &cap, copy task, digest, @0x52, test_scenario::ctx(&mut scenario));
        test_scenario::return_shared(registry);
        test_scenario::return_to_sender(&scenario, cap);
        test_scenario::next_tx(&mut scenario, operator);

        let registry = test_scenario::take_shared<Registry>(&scenario);
        let cap = test_scenario::take_from_sender<IssuerCap>(&scenario);
        issue(&mut registry, &cap, task, replay_digest, replay_recipient, test_scenario::ctx(&mut scenario));
        assert!(registry.issuance_count == 1, 4);
        test_scenario::return_shared(registry);
        test_scenario::return_to_sender(&scenario, cap);
        test_scenario::end(scenario);
    }

    #[test]
    fun same_binding_replay_preserves_issuance_count() {
        check_replay(x"2222222222222222222222222222222222222222222222222222222222222222", @0x52);
    }

    #[test]
    #[expected_failure(abort_code = EBindingMismatch)]
    fun rejects_changed_digest() {
        check_replay(x"3333333333333333333333333333333333333333333333333333333333333333", @0x52);
    }

    #[test]
    #[expected_failure(abort_code = EBindingMismatch)]
    fun rejects_changed_recipient() {
        check_replay(x"2222222222222222222222222222222222222222222222222222222222222222", @0x53);
    }
}
