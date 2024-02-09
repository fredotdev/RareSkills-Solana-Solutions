import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day_2";

describe("day_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(new anchor.BN(777), new anchor.BN(888), "hello")
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Array test", async () => {
    const tx = await program.methods
      .array([new anchor.BN(777), new anchor.BN(888)])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // exercise 1 and 2
  it("Sub test", async () => {
    const tx = await program.methods
      .sub(new anchor.BN(0), new anchor.BN(1))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // exercise 3
  it("Checked sub test", async () => {
    const tx = await program.methods
      .checkedsub(new anchor.BN(0), new anchor.BN(1))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // exercise 4 from here on
  it("Checked sqrt test", async () => {
    const tx = await program.methods.sqrt(new anchor.BN(4)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Checked log test", async () => {
    const tx = await program.methods.log(new anchor.BN(4)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Checked div test", async () => {
    const tx = await program.methods
      .div(new anchor.BN(4), new anchor.BN(2))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Checked mul test", async () => {
    const tx = await program.methods
      .mul(new anchor.BN(4), new anchor.BN(4))
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Checked add test", async () => {
    const tx = await program.methods
      .add(new anchor.BN(4), new anchor.BN(4))
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
