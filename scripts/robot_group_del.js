const {
    exec
} = require("child_process");
let group_id = "a14d7d2a-3e4d-4aaf-bce5-9cecd8192897";
let robots_ids = ['82d274e9-a690-4b48-9174-233b2526039a', 'ce733ffa-5404-420c-aecc-5fccfa49ade3', '25f952a9-8741-4636-a7d3-1de943a27dd1', '1099ea61-18c1-4675-90d1-f42933d762f9', '2133f8b4-5bb1-40f2-a762-8ae716d3c026', 'c581106b-dbed-4d6e-96b6-f65214350404', '054374b3-0705-459c-b092-1bb9934fe7a4', 'f2888c2e-0470-41bc-a646-5eafe517899a', '94418edf-1afc-4779-8fb3-eb30353c0387', '6f98d412-30ad-40ff-ab04-b000a0d716aa', '2fba3160-397c-4835-a3f4-2cd54ab9c3f5', '2d8f8267-0d52-4b7a-9ee1-1bdc1575d9f7', '857a12c2-4ab8-4ab7-897b-ba0aad0ec7a1', '2e11f44e-5719-43db-bf55-8363895ea312', 'b792ba40-1411-4302-b40d-21bc84cfb860', '7b7f3416-1734-4bb2-850e-2217b64b0bbf', '664a3219-6c3e-43b9-87d8-07d247fda1a5', '16e08105-e4ae-4418-983f-e100522203ef', 'b152e07c-af3b-45e6-9dc2-d1d2dd7734cf', '71c87d61-9a29-41e6-afb3-0504a3cc96b6', '23fd70ae-b10f-4a3b-8d9e-99209906dcf2', '58689eb2-d8cb-4e7e-9744-fbaa990013ea', '1a113ff7-9cf8-41be-8a75-09e506d37cfd', 'aa39f7b8-9964-4cb2-a9f4-c82f1b17c3c6', '05c0acfc-fa9f-4c2f-8082-eb52db71a2d1', '9e7bcd35-7bab-4963-a830-69bc82ecaefa'];

robots_ids.forEach(robot_id => {
    let cmd = `npm run aker-gql-op -- robot_group_del  --group_id "${group_id}" --robot_id "${robot_id}"`;
    console.log(cmd);

    exec(cmd, (error, stdout, stderr) => {
        if (error) {
            console.log(`error: ${error.message}`);
            return;
        }
        if (stderr) {
            console.log(`stderr: ${stderr}`);
            return;
        }
        console.log(`stdout: ${stdout}`);
    });
})