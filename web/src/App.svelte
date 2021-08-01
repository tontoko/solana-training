<script lang="ts">
  import logo from "./assets/svelte.png";
  import Counter from "./lib/Counter.svelte";
  import {
    TransactionInstruction,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction,
    Connection,
    Keypair,
    SystemProgram,
    LAMPORTS_PER_SOL,
  } from "@solana/web3.js";
  import { serialize, deserialize, Schema } from "borsh";

  const programId = "6C8UrN683UrUSnkXQRTqnUb6DyJMvVf1gVujDNfNQ4ou";
  let greetedPubkey: PublicKey;

  class GreetingAccount {
    counter = 0;
    constructor(fields: { counter: number } | undefined = undefined) {
      if (fields) {
        this.counter = fields.counter;
      }
    }
  }
  const GreetingSchema: Schema = new Map([
    [GreetingAccount, { kind: "struct", fields: [["counter", "u32"]] }],
  ]);
  const GREETING_SIZE = serialize(GreetingSchema, new GreetingAccount()).length;

  const test = async (
    connection: Connection,
    account: Keypair,
    greetedPubkey: PublicKey
  ) => {
    const instruction = new TransactionInstruction({
      keys: [
        {
          pubkey: greetedPubkey,
          isSigner: false,
          isWritable: true,
        },
      ],
      // sampleプログラムのID
      programId: new PublicKey(programId),
      data: Buffer.alloc(0),
    });
    try {
      await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [account]
      );
    } catch (e) {
      return console.log("error", e);
    }
    console.log("done");
  };

  async function reportGreetings(connection, greetedPubkey): Promise<void> {
    const accountInfo = await connection.getAccountInfo(greetedPubkey);
    if (accountInfo === null) {
      throw "Error: cannot find the greeted account";
    }
    const greeting = deserialize(
      GreetingSchema,
      GreetingAccount,
      accountInfo.data
    );
    console.log(
      greetedPubkey.toBase58(),
      "has been greeted",
      greeting.counter,
      "time(s)"
    );
  }

  const main = async () => {
    // devnetのコネクション作成
    let connection = new Connection(
      "https://api.devnet.solana.com",
      "confirmed"
    );
    console.log("connected");
    // プログラム実行するためのアカウント作成
    const payer = Keypair.generate();
    const payerSignature = await connection.requestAirdrop(
      payer.publicKey,
      LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(payerSignature);
    console.log("airdrop done");

    const GREETING_SEED = "hello";

    if (!greetedPubkey) {
      greetedPubkey = await PublicKey.createWithSeed(
        payer.publicKey,
        GREETING_SEED,
        new PublicKey(programId)
      );
    }

    const greetedAccount = await connection.getAccountInfo(greetedPubkey);
    if (greetedAccount === null) {
      console.log(
        "Creating account",
        greetedPubkey.toBase58(),
        "to say hello to"
      );
      const lamports = await connection.getMinimumBalanceForRentExemption(
        GREETING_SIZE
      );

      const transaction = new Transaction().add(
        SystemProgram.createAccountWithSeed({
          fromPubkey: payer.publicKey,
          basePubkey: payer.publicKey,
          seed: GREETING_SEED,
          newAccountPubkey: greetedPubkey,
          lamports,
          space: GREETING_SIZE,
          programId: new PublicKey(programId),
        })
      );
      await sendAndConfirmTransaction(connection, transaction, [payer]);
    }

    console.log("account:", greetedPubkey.toBase58());

    await test(connection, payer, greetedPubkey);
    await reportGreetings(connection, greetedPubkey);
  };
</script>

<main>
  <img src={logo} alt="Svelte Logo" />
  <h1>Hello Typescript!</h1>

  <Counter />

  <div>
    <h2>say hello</h2>
    <button on:click={main}>実行</button>
  </div>

  <p>
    Visit <a href="https://svelte.dev">svelte.dev</a> to learn how to build Svelte
    apps.
  </p>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme">SvelteKit</a> for
    the officially supported framework, also powered by Vite!
  </p>
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }

  img {
    height: 16rem;
    width: 16rem;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4rem;
    font-weight: 100;
    line-height: 1.1;
    margin: 2rem auto;
    max-width: 14rem;
  }

  p {
    max-width: 14rem;
    margin: 1rem auto;
    line-height: 1.35;
  }

  @media (min-width: 480px) {
    h1 {
      max-width: none;
    }

    p {
      max-width: none;
    }
  }
</style>
