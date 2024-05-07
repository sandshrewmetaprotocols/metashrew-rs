
import { expect } from "chai";
import fs from "fs-extra";
import { EventEmitter } from "events";
import { IndexerProgram } from "metashrew-test";
import path from "path";


describe("metashrew index", () => {
  it("indexes the genesis block", async () => {
    const program = new IndexerProgram(
      new Uint8Array(
        Array.from(
          await fs.readFile(
            path.join("pkg", "metashrew_rs_bg.wasm")
          ),
        ),
      ).buffer,
    );
    program.setBlock(
      await fs.readFile(path.join(__dirname, "genesis.hex"), "utf8"),
    );
    program.setBlockHeight(0);
    program.on("log", (v) => process.stdout.write(v));
    await program.run('_test');
    console.log(program.kv);
    // console.log(program.kv);
  });
});
