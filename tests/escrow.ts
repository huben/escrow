import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
} from '@solana/web3.js'
import {
  TOKEN_PROGRAM_ID,
  getAccount,
} from '@solana/spl-token'
import * as serumCmn from "@project-serum/common"
import { assert } from 'chai'

import { Escrow } from "../target/types/escrow";

describe("escrow", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Escrow as Program<Escrow>;

  let creatorTokenAccountX: PublicKey = null
  let creatorTokenAccountY: PublicKey = null

  let takerTokenAccountX: PublicKey = null
  let takerTokenAccountY: PublicKey = null

  let expense = 5
  let expect = 10

  const escrowAccount = anchor.web3.Keypair.generate()
  const payer = anchor.web3.Keypair.generate()

  it("Is initialized!", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 100000000000)
    )

    const serumCmnProvider = new serumCmn.Provider(provider.connection, provider.wallet, provider.opts);
    // 创建币厂 x，同时给 creator 铸 expense 个币
    let [ mintX, accountX ] = await serumCmn.createMintAndVault(serumCmnProvider, new BN(expense))
    creatorTokenAccountX = accountX
    // 给 taker 创建一个 x 币的账户，将接受 creator 的 x 币
    takerTokenAccountX = await serumCmn.createTokenAccount(serumCmnProvider, mintX, provider.wallet.publicKey)

    // 创建币厂 Y，同时给 taker 铸 expect 个币
    let [ mintY, accountY ] = await serumCmn.createMintAndVault(serumCmnProvider, new BN(expect))
    takerTokenAccountY = accountY
    // 给 creator 创建一个 y 币的账户，将接受 taker 的 y 币
    creatorTokenAccountY = await serumCmn.createTokenAccount(serumCmnProvider, mintY, provider.wallet.publicKey)
    
    const _creatorTokenAccountXInfo = await getAccount(provider.connection, creatorTokenAccountX)
    const _creatorTokenAccountYInfo = await getAccount(provider.connection, creatorTokenAccountY)
    assert.strictEqual(_creatorTokenAccountXInfo.amount, BigInt(expense));
    assert.strictEqual(_creatorTokenAccountYInfo.amount, BigInt(0));
    
    const _takerTokenAccountXInfo = await getAccount(provider.connection, takerTokenAccountX)
    const _takerTokenAccountYInfo = await getAccount(provider.connection, takerTokenAccountY)
    assert.strictEqual(_takerTokenAccountXInfo.amount, BigInt(0));
    assert.strictEqual(_takerTokenAccountYInfo.amount, BigInt(expect));
  
    assert.isTrue(_creatorTokenAccountXInfo.owner.equals(provider.wallet.publicKey));
    
  });

  it("create escrow", async () => {
    await program.rpc.createEscrow(
      new BN(expense),
      new BN(expect),
      {
        accounts: {
          escrowAccount: escrowAccount.publicKey,
          signer: provider.wallet.publicKey,
          expenseTokenAccount: creatorTokenAccountX,
          expectTokenAccount: creatorTokenAccountY,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [escrowAccount],
      },
    )

    const _escrowAccountInfo = await program.account.escrowAccount.fetch(escrowAccount.publicKey)
    
    const [pda, bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode('escrow_pda_seed'))], 
      program.programId
    )

    assert.isTrue(_escrowAccountInfo.authority.equals(provider.wallet.publicKey))
    assert.isTrue(_escrowAccountInfo.expenseTokenAccount.equals(creatorTokenAccountX))
    assert.isTrue(_escrowAccountInfo.expectTokenAccount.equals(creatorTokenAccountY))
    assert.strictEqual(_escrowAccountInfo.expenseAmount.toNumber(), expense)
    assert.strictEqual(_escrowAccountInfo.expectAmount.toNumber(), expect)
    assert.isTrue(_escrowAccountInfo.pda.equals(pda));
    
    const _creatorTokenAccountXInfo = await getAccount(provider.connection, creatorTokenAccountX)
    
    assert.isTrue(_creatorTokenAccountXInfo.owner.equals(pda));
    
  })

  it("swap escrow", async () => {
    const _escrowAccountInfo = await program.account.escrowAccount.fetch(escrowAccount.publicKey)
    
    await program.rpc.swapEscrow(
      {
        accounts: {
          escrowAccount: escrowAccount.publicKey,
          taker: provider.wallet.publicKey,
          takerTokenAccountX: takerTokenAccountX,
          takerTokenAccountY: takerTokenAccountY,
          creatorTokenAccountX: creatorTokenAccountX,
          creatorTokenAccountY: creatorTokenAccountY,
          pdaAccount: _escrowAccountInfo.pda,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
      },
    )

    const _creatorTokenAccountXInfo = await getAccount(provider.connection, creatorTokenAccountX)
    const _creatorTokenAccountYInfo = await getAccount(provider.connection, creatorTokenAccountY)
    assert.strictEqual(_creatorTokenAccountXInfo.amount, BigInt(0));
    assert.strictEqual(_creatorTokenAccountYInfo.amount, BigInt(expect));

    const _takerTokenAccountXInfo = await getAccount(provider.connection, takerTokenAccountX)
    const _takerTokenAccountYInfo = await getAccount(provider.connection, takerTokenAccountY)
    assert.strictEqual(_takerTokenAccountXInfo.amount, BigInt(expense));
    assert.strictEqual(_takerTokenAccountYInfo.amount, BigInt(0));

    assert.isTrue(_creatorTokenAccountXInfo.owner.equals(provider.wallet.publicKey));
  })
});
