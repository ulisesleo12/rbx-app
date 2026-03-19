const {exec} = require("child_process");
const fs = require('fs');


let rawdata = fs.readFileSync('robots_ids.json');
let robots = JSON.parse(rawdata);

let robot_name = "K3";

robots.data.robot_profile.forEach(obj => {

if(obj.name.substring(0,2) == robot_name){
    console.log(obj.name);
    let new_name = "PP"+obj.name.substring(2,);

    let cmd = `npm run aker-gql-op -- update_robot_name_by_id  --robot_id ${obj.robot_id} --name ${new_name}`;
          console.log(cmd);

        //   exec(cmd, (error, stdout, stderr) => {
        //       if (error) {
        //           console.log(`error: ${error.message}`);
        //           return;
        //       }
        //       if (stderr) {
        //           console.log(`stderr: ${stderr}`);
        //           return;
        //       }
        //       console.log(`stdout: ${stdout}`);
        //   });
    
}
});
