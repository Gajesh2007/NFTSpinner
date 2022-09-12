import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftSpinner } from "../target/types/nft_spinner";

describe("nft-spinner", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftSpinner as Program<NftSpinner>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
