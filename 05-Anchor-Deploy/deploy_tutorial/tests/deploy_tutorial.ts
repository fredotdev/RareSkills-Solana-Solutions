import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import fs from "fs";
let idl = JSON.parse(
  fs.readFileSync("target/idl/deploy_tutorial.json", "utf-8")
);

describe("deployed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const programID = "5R9eQnyNB5gRoLK1ceqE8cc74BzvzMYxTfbZUpN27psa";
  const program = new Program(idl, programID, anchor.getProvider());

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
