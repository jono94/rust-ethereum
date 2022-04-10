

// 4.1. World State.
//
// The world state (state), is a mapping between addresses (160-bit identifiers) and account
// states (a data structure serialised as RLP, see Appendix B).
// Though not stored on the blockchain, it is assumed that
// the implementation will maintain this mapping in a modified Merkle Patricia tree (trie, see Appendix D). The trie
// requires a simple database backend that maintains a mapping of byte arrays to byte arrays; we name this underlying
// database the state database. This has a number of benefits;
// firstly the root node of this structure is cryptographically
// dependent on all internal data and as such its hash can
// be used as a secure identity for the entire system state.
// Secondly, being an immutable data structure, it allows any
// previous state (whose root hash is known) to be recalled
// by simply altering the root hash accordingly. Since we
// store all such root hashes in the blockchain, we are able to
// trivially revert to old states.
//
// The account state, σ[a], comprises the following four
// fields:
//
//     nonce: A scalar value equal to the number of transactions sent from this address or, in the case
//         of accounts with associated code, the number of
//         contract-creations made by this account. For account of address a in state σ, this would be formally denoted σ[a]n.
//     balance: A scalar value equal to the number of Wei
//         owned by this address. Formally denoted σ[a]b.
//     storageRoot: A 256-bit hash of the root node of a
//         Merkle Patricia tree that encodes the storage contents of the account (a mapping between 256-bit
//         integer values), encoded into the trie as a mapping
//         from the Keccak 256-bit hash of the 256-bit integer
//         keys to the RLP-encoded 256-bit integer values.
//         The hash is formally denoted σ[a]s.
//     codeHash: The hash of the EVM code of this
//         account—this is the code that gets executed should
//         this address receive a message call; it is immutable
//         and thus, unlike all other fields, cannot be changed
//         after construction. All such code fragments are
//         contained in the state database under their corresponding hashes for later retrieval. This hash is
//         formally denoted σ[a]c, and thus the code may be
//         denoted as b, given that KEC(b) = σ[a]c.
struct AccountState {
    nonce: u128,
    balance: u128,
    storageRoot: String,
    codeHash: String,
}

// 4.2. The Transaction.
//
// A transaction (formally, T) is a
// single cryptographically-signed instruction constructed by
// an actor externally to the scope of Ethereum. The sender
// of a transaction cannot be a contract. While it is assumed
// that the ultimate external actor will be human in nature,
// software tools will be used in its construction and dissemination1
// . EIP-2718 by Zoltu [2020] introduced the notion
// of different transaction types. As of the Berlin version of
// the protocol, there are two transaction types: 0 (legacy)
// and 1 (EIP-2930 by Buterin and Swende [2020b]). Further,
// there are two subtypes of transactions: those which result
// in message calls and those which result in the creation of
// new accounts with associated code (known informally as
// ‘contract creation’). All transaction types specify a number
// of common fields:
//
//     type: EIP-2718 transaction type; formally Tx.
//     nonce: A scalar value equal to the number of transactions sent by the sender; formally Tn.
//     gasPrice: A scalar value equal to the number of
//         Wei to be paid per unit of gas for all computation
//         costs incurred as a result of the execution of this
//         transaction; formally Tp.
//     gasLimit: A scalar value equal to the maximum
//         amount of gas that should be used in executing
//         this transaction. This is paid up-front, before any
//         computation is done and may not be increased
//         later; formally Tg.
//     to: The 160-bit address of the message call’s recipient or, for a contract creation transaction, ∅, used
//         here to denote the only member of B0 ; formally
//         Tt.
//     value: A scalar value equal to the number of Wei to
//         be transferred to the message call’s recipient or,
//         in the case of contract creation, as an endowment
//         to the newly created account; formally Tv.
//     r, s: Values corresponding to the signature of the
//         transaction and used to determine the sender of
//         the transaction; formally Tr and Ts. This is expanded in Appendix F.
//
// EIP-2930 (type 1) transactions also have:
//
//     accessList: List of access entries to warm up; formally TA. Each access list entry E is a tuple
//         of an account address and a list of storage keys:
//         E ≡ (Ea, Es).
//     chainId: Chain ID; formally Tc. Must be equal to
//         the network chain ID β.
//     yParity: Signature Y parity; formally Ty.
//
// Legacy transactions do not have an accessList (TA =
// ()), while chainId and yParity for legacy transactions
// are combined into a single value:
//
//     w: A scalar value encoding Y parity and possibly chain ID; formally Tw. Tw = 27 + Ty or
//         Tw = 2β+35+Ty (see EIP-155 by Buterin [2016b]).
//
// Additionally, a contract creation transaction (regardless
// whether legacy or EIP-2930) contains:
//
//     init: An unlimited size byte array specifying the
//         EVM-code for the account initialisation procedure,
//         formally Ti.
//
// init is an EVM-code fragment; it returns the body,
// a second fragment of code that executes each time the
// account receives a message call (either through a transaction or due to the internal execution of code). init is
// executed only once at account creation and gets discarded
// immediately thereafter.
//
// In contrast, a message call transaction contains:
//
//     data: An unlimited size byte array specifying the
//         input data of the message call, formally Td.
struct Transaction {
    r#type: u128,
    nonce: u128,
    gasPrice: u128,
    gasLimit: u128,
    to: u128,
    value: u128,
}

struct EIP2930Trasaction { // + Transaction
    accessList: Vec<u128>,
    chainId: u8,
    yParity: u128,
}

struct LegacyTransaction { // + Transaction
    w: u128,
}

struct ContractCreationTransaction { // + EIP2930Transaction or LegacyTransaction
    init: String,
}

struct MessageCallTransaction { // + EIP2930Transaction or LegacyTransaction
    data: String,
}


// 4.3. The Block.
//
// The block in Ethereum is the collection of relevant pieces of information (known as the block
// header ), H, together with information corresponding to
// the comprised transactions, T, and a set of other block
// headers U that are known to have a parent equal to the
// present block’s parent’s parent (such blocks are known as
// ommers2
// ). The block header contains several pieces of
// information:
//
//    parentHash: The Keccak 256-bit hash of the parent
//         block’s header, in its entirety; formally Hp.
//    ommersHash: The Keccak 256-bit hash of the ommers list portion of this block; formally Ho.
//    beneficiary: The 160-bit address to which all fees
//         collected from the successful mining of this block
//         be transferred; formally Hc.
//    stateRoot: The Keccak 256-bit hash of the root
//         node of the state trie, after all transactions are
//         executed and finalisations applied; formally Hr.
//    transactionsRoot: The Keccak 256-bit hash of the
//         root node of the trie structure populated with each
//         transaction in the transactions list portion of the
//         block; formally Ht.
//    receiptsRoot: The Keccak 256-bit hash of the root
//         node of the trie structure populated with the receipts of each transaction in the transactions list
//         portion of the block; formally He.
//    logsBloom: The Bloom filter composed from indexable information (logger address and log topics)
//         contained in each log entry from the receipt of
//         each transaction in the transactions list; formally
//         Hb.
//    difficulty: A scalar value corresponding to the difficulty level of this block. This can be calculated
//         from the previous block’s difficulty level and the
//         timestamp; formally Hd.
//    number: A scalar value equal to the number of ancestor blocks. The genesis block has a number of
//         zero; formally Hi.
//    gasLimit: A scalar value equal to the current limit
//         of gas expenditure per block; formally Hl.
//    gasUsed: A scalar value equal to the total gas used
//         in transactions in this block; formally Hg.
//    timestamp: A scalar value equal to the reasonable
//         output of Unix’s time() at this block’s inception;
//         formally Hs.
//    extraData: An arbitrary byte array containing data
//         relevant to this block. This must be 32 bytes or
//         fewer; formally Hx.
//    mixHash: A 256-bit hash which, combined with the
//         nonce, proves that a sufficient amount of computation has been carried out on this block; formally
//         Hm.
//    nonce: A 64-bit value which, combined with the mixhash, proves that a sufficient amount of computation
//         has been carried out on this block; formally
//         Hn.
//
// The other two components in the block are simply a list
// of ommer block headers (of the same format as above),
// BU and a series of the transactions, BT. Formally, we can
// refer to a block B:
//
//     (21) B ≡ (BH, BT, BU)
struct Block {
    parentHash: String,
    ommersHash: String,
    beneficiary: u128,
    stateRoot: String,
    transactionRoot: String,
    receiptsRoot: String,
    logsBloom: u128,
    difficulty: u128,
    number: u128,
    gasLimit: u128,
    gasUsed: u128,
    timestamp: u128,
    extraData: String,
    mixHash: String,
    nonce: u64,
}

// 4.3.1. Transaction Receipt.

// 4.3.2. Holistic Validity.

// 4.3.3. Serialisation.

// 4.3.4. Block Header Validity.

