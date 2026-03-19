const {exec} = require("child_process");
const fs = require('fs');

// Upload pic_profile
directory_pic = '../../../avatares'
        
fs.readdir(directory_pic, function (err, files) {
    if (err) {
        onError(err);
        return;
    }   
    // console.log(files);
    for(file in files){

        cmd = `scp -r ${directory_pic}${files[file]} aker-service:/var/www/files.aker.network/html/pic_profile/${files[file]}`;

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
});

// Add teachers

group_id = '35788672-e94d-437c-8daf-22f6021a8734';
inventory_group_id = 'f1a350e6-5f43-455b-a1ce-85dafafa38eb';


fs.readFile('./teachers.csv', 'utf8', function (err, data) {
    var data = data.split(/\r?\n/);
    
    for(name in data){
        array = data[name].split(',');
        
      user_id = array[5];
      full_name = array[0]+' '+array[1]+' '+array[2]+' '+array[3];

      cmd = `npm run aker-gql-op -- user_teacher_group_add --full_name "${full_name}" --pic_path "https://files.aker.network/pic_profile/${array[4]}" --group_id ${group_id} --inventory_group_id ${inventory_group_id} --user_id ${user_id}`;

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

});