const {exec} = require("child_process");
const fs = require('fs');

directory = '../../../aker-robots/robots';

robot_inventor_group = '';
robot_id = '';
group_id = '';

fs.readdir(directory, function (err, files) {
    if (err) {
        onError(err);
        return;
    }   

    for(i in files){
        fileName = files[i].split(".");
        if(fileName[1]=='glb'){
            cmd = `npm run aker-gql-op -- robot_group_create --name ${fileName[0]} --path ${fileName[0]} --robot_inventor_group ${robot_inventor_group} --group_id ${group_id} --robot_id ${robot_id}`;
            console.log(cmd)

        //     exec(cmd, (error, stdout, stderr) => {
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
   }

})