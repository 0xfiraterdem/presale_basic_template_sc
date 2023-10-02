#![no_std]

multiversx_sc::imports!();

pub const TOTAL_PERCENTAGE: u64 = 1_000_000_000_000_000_000u64;


#[multiversx_sc::contract]
pub trait PresaleContract {
    #[init]
    fn init(
        &self,
        price_per_token: BigUint,
        total_tokens: BigUint,
        payment_token_id: EgldOrEsdtTokenIdentifier,
    ) {
        self.price_per_token().set(&price_per_token);
        self.total_tokens().set(&total_tokens);
        self.payment_token_id().set(&payment_token_id); 
    }

    #[payable("EGLD")]
    #[endpoint(preSale)]
    fn buy_tokens(&self, #[payment_amount] payment_amount: BigUint) -> SCResult<()> {
        let payment_token_id = self.payment_token_id().get();
        let price_per_token = self.price_per_token().get();
        let tokens_to_buy = payment_amount / price_per_token;
        let total_tokens = self.total_tokens().get();

        require!(tokens_to_buy <= total_tokens, "Not enough tokens available {}", tokens_to_buy);

        let tokens_to_send = tokens_to_buy.clone() * BigUint::from(TOTAL_PERCENTAGE); // TOTAL_PERCENTAGE sabitini BigUint türüne çevirerek çarpma işlemi gerçekleştirilir.
        
        self.total_tokens().set(&(total_tokens - tokens_to_buy));
        
        let caller = self.blockchain().get_caller();
        self.send().direct(&caller, &payment_token_id, 0, &tokens_to_send);

        Ok(())
    }
    
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint]
    fn withdraw(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let contract_balance = self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0);
        self.send().direct(&caller, &EgldOrEsdtTokenIdentifier::egld(), 0, &contract_balance);
        Ok(())
    }
    #[only_owner]
    #[payable("Your Token Id")] 
    #[endpoint]
    fn deposit(&self, 
        #[payment_token] token_id: EgldOrEsdtTokenIdentifier, 
        #[payment_amount] amount: BigUint) -> SCResult<()> {
        let expected_token_id = TokenIdentifier::from("Your Token Id");
        require!(token_id == expected_token_id, "Invalid token type");
        Ok(())
    }

    #[view(getPricePerToken)]
    #[storage_mapper("pricePerToken")]
    fn price_per_token(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalTokens)]
    #[storage_mapper("totalTokens")]
    fn total_tokens(&self) -> SingleValueMapper<BigUint>;

    #[view(getAcceptedTokenId)]
    #[storage_mapper("acceptedTokenId")]
    fn payment_token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;
}
