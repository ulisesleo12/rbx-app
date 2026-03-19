const {exec} = require("child_process");

// User-ids
K1 = [21, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]


group_id = '';

for(let i=0; i<K1.length; i++){

    cmd = `npm run aker-gql-op -- group_member_add --group_id ${group_id} --user_id ${K1[i]}`;
    console.log(cmd);

    // exec(cmd, (error, stdout, stderr) => {
    //     if (error) {
    //         console.log(`error: ${error.message}`);
    //         return;
    //     }
    //     if (stderr) {
    //         console.log(`stderr: ${stderr}`);
    //         return;
    //     }
    //     console.log(`stdout: ${stdout}`);
    // });
}