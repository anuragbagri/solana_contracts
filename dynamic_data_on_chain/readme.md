# Solana: Storing Dynamic Data On-Chain

This guide covers two common patterns for storing dynamic or variable-length data in Solana programs.

---

## 1. Preallocate Enough Space (`fixed_size.rs`)

If you can predict the maximum size your data might grow to, you can create a single account with enough space.

**Pros:**

- Simple logic
- Only one account to manage

**Cons:**

- Wastes space if unused
- Expensive if the allocated space is too large

---

## 2. Use Multiple Accounts (One Base + Dynamic Children) (`dynamic.rs`)

Split your data across multiple accounts:

- **Main account:** Stores metadata or summary information
- **Child accounts:** Each stores an item or a chunk of the data

**Pros:**

- Efficient use of space (only pay for what you use)
- Can scale to large or unpredictable data sizes

**Cons:**

- More complex logic (must manage multiple accounts)
- Higher transaction complexity and potential rent costs

---

Choose the approach that best fits your application's needs
