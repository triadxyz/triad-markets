use anchor_lang::prelude::*;

#[error_code]
pub enum TriadProtocolError {
    #[msg("Invalid account")]
    InvalidAccount,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Failed to deposit")]
    DepositFailed,

    #[msg("Invalid Owner authority")]
    InvalidOwnerAuthority,

    #[msg("Invalid Position")]
    InvalidPosition,

    #[msg("Invalid Ticker position")]
    InvalidTickerPosition,

    #[msg("No free position slot")]
    NoFreePositionSlot,

    #[msg("Invalid Mint address")]
    InvalidMintAddress,

    #[msg("Invalid Profit Share")]
    InvalidProfitShare,

    #[msg("Invalid Deposit Amount")]
    InvalidDepositAmount,

    #[msg("Invalid Withdraw Amount")]
    InvalidWithdrawAmount,

    #[msg("Invalid Stake Vault")]
    InvalidStakeVault,

    #[msg("Invalid Stake Vault Authority")]
    InvalidStakeVaultAuthority,

    #[msg("Invalid Stake Vault Amount")]
    InvalidStakeVaultAmount,

    #[msg("Stake Vault Available")]
    StakeVaultLocked,

    #[msg("Stake is locked")]
    StakeLocked,

    #[msg("Stake Vault Full")]
    StakeVaultFull,

    #[msg("Invalid Mint")]
    InvalidMint,

    #[msg("Invalid Stake Vault Week")]
    InvalidStakeVaultWeek,
}
