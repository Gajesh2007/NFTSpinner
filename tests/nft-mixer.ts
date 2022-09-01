import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftMixer } from "../target/types/nft_mixer";

describe("nft-mixer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftMixer as Program<NftMixer>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
