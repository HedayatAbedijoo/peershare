import {
  Orchestrator,
  Config,
  InstallAgentsHapps,
  InstalledAgentHapps,
} from "@holochain/tryorama";
import { ScenarioApi } from "@holochain/tryorama/lib/api";
import path from "path";

const conductorConfig = Config.gen();

const conductorHapps: InstallAgentsHapps = [
  // agent 0 ...
  [
    // happ 0
    [
      // dna 0
      path.join("../peershare.dna.gz"),
    ],
  ],
];

const orchestrator = new Orchestrator();
/*
orchestrator.registerScenario(
  "Scenario: Tag Validation",
  async (s: ScenarioApi, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);
    const [alice_test_happ] = await alice.installAgentsHapps(conductorHapps);
    const Tag_ZOME_NAME = "tags";
    const FileStorage_Zome_Name = "file_storage";
    const Dummy_Hash_File =
      "uhCEkM2EO2T5APtzYYoLWZYkkOeLDWSJQIUYLGjvgPBlmknFGLz30";
    const conductor = alice_test_happ[0].cells[0];

    /// invalid: because of number in tag
    let test1 = await alice_test_happ[0].cells[0].call(
      Tag_ZOME_NAME,
      "tag_file",
      {
        file_hash: Dummy_Hash_File,
        tags: ["1ewerr"],
      }
    );
    t.ok(test1);
    t.deepEqual(test1.result, false);
    _log(test1, "tag_file_result");

    /// invalid: because special character
    test1 = await alice_test_happ[0].cells[0].call(Tag_ZOME_NAME, "tag_file", {
      file_hash: Dummy_Hash_File,
      tags: ["_sdfw"],
    });
    t.ok(test1);
    t.deepEqual(test1.result, false);
    _log(test1, "tag_file_result");

    /// invalid: because of lenght <3
    test1 = await alice_test_happ[0].cells[0].call(Tag_ZOME_NAME, "tag_file", {
      file_hash: Dummy_Hash_File,
      tags: ["er"],
    });
    t.ok(test1);
    t.deepEqual(test1.result, false);
    _log(test1, "tag_file_result");

    /// invalid: because of lenght > 10
    test1 = await alice_test_happ[0].cells[0].call(Tag_ZOME_NAME, "tag_file", {
      file_hash: Dummy_Hash_File,
      tags: ["erfdvgbnhtg"],
    });
    t.ok(test1);
    t.deepEqual(test1.result, false);
    _log(test1, "tag_file_result");
  }
);
*/
orchestrator.registerScenario(
  "Scenario: Test Upload and Tag",
  async (s: ScenarioApi, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);
    const [alice_test_happ] = await alice.installAgentsHapps(conductorHapps);
    const Tag_ZOME_NAME = "tags";
    const FileStorage_Zome_Name = "file_storage";
    const PeerShare_Zome_Name = "peershare";
    const conductor = alice_test_happ[0].cells[0];

    const fileMetadata = dummy_file_metadata();
    let fileHash = await conductor.call(
      FileStorage_Zome_Name,
      "create_file_metadata",
      fileMetadata
    );
    t.ok(fileHash);
    _log(fileHash, "file created");

    /// upload a new meta_data file
    let test1 = await conductor.call(Tag_ZOME_NAME, "tag_file", {
      file_hash: fileHash,
      tags: ["movie"],
    });
    t.ok(test1);
    t.deepEqual(test1.result, true);
    _log(test1, "file_tagged");

    ///// Test link to my address is working.
    let myfilesResult = await conductor.call(
      Tag_ZOME_NAME,
      "get_my_files",
      null
    );

    t.ok(myfilesResult);
    t.deepEqual(myfilesResult.list, 1);
    _log(myfilesResult, "all my files");
  }
);

orchestrator.registerScenario(
  "Scenario: Test new extension function to file_storage module",
  async (s: ScenarioApi, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig]);
    const [alice_test_happ] = await alice.installAgentsHapps(conductorHapps);
    const Tag_ZOME_NAME = "tags";
    const FileStorage_Zome_Name = "file_storage";
    const PeerShare_Zome_Name = "peershare";
    const conductor = alice_test_happ[0].cells[0];

    /////
    let file_stortage_test = await conductor.call(
      FileStorage_Zome_Name,
      "new_extention_function",
      null
    );

    t.ok(file_stortage_test);
    _log(file_stortage_test, "test file storage new function");
  }
);

function dummy_file_metadata() {
  const chunksHashes: any[] = [];
  return {
    name: "dummy.txt",
    fileType: "text/plain",
    chunksHashes,
    size: 1024,
    lastModified: [Math.floor(Date.now() / 1000), 0],
  };
}

// orchestrator.registerScenario(
//   "create file test scenario",
//   async (s: ScenarioApi, t) => {
//     const [alice, bob] = await s.players([conductorConfig, conductorConfig]);
//     const [alice_test_happ] = await alice.installAgentsHapps(conductorHapps);
//     const ZOME_NAME = "file_storage";
//     const conductor = alice_test_happ[0].cells[0];

//     // In memory dummy file to upload to DNA
//     const chunkSize = 8 * 1024;
//     const chunkNumer = 6;
//     const bufStr = Array(chunkSize).fill("h").join("");
//     let chunkBytes = Buffer.from(bufStr, "utf8");
//     const chunksHashes: any[] = [];

//     ////////Upload each chunk as a file_chunk
//     for (let i = 0; i < chunkNumer; i++) {
//       const start = Date.now();
//       const hash = await conductor.call(
//         ZOME_NAME,
//         "create_file_chunk",
//         Buffer.from(Array(chunkSize).fill(i).join(""))
//       );
//       const end = Date.now();
//       console.log(chunkBytes.length);
//       console.log((end - start) / 1000);
//       chunksHashes.push(hash);
//     }
//     console.log("File Hashes:**********");
//     console.log(chunksHashes);
//     console.log("*********************");

//     /////// Upload file Metadata to DNA
//     let fileMetadata = {
//       name: "example.txt",
//       fileType: "text/plain",
//       chunksHashes,
//       size: chunkSize * chunkNumer,
//       lastModified: [Math.floor(Date.now() / 1000), 0],
//     };
//     let fileHash = await conductor.call(
//       ZOME_NAME,
//       "create_file_metadata",
//       fileMetadata
//     );
//     t.ok(fileHash);

//     console.log("MetaData file Hashe:**********");
//     console.log(fileMetadata);
//     console.log("*********************");

//     let fileResult = await conductor.call(
//       ZOME_NAME,
//       "get_file_metadata",
//       fileHash
//     );
//     t.ok(fileResult);

//     for (const chunkHash of fileResult.chunksHashes) {
//       let chunk = await conductor.call(ZOME_NAME, "get_file_chunk", chunkHash);
//       t.ok(chunk);
//       console.log(chunk);
//     }
//   }
// );

orchestrator.run();

function _log(value, title) {
  console.log("====>" + title + "<=======");
  console.log(value);
  console.log("______________");
}
