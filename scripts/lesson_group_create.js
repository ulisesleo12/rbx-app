const {exec} = require("child_process");
const fs = require('fs');

let grade = "K5"

const dir = `../../LECCIONES/${grade}`;
let author_id = "ef7451c8-2e7e-4438-99f1-701e3353b768"

// fs.readdir(dir, (err, files) => {
//   files.forEach(file => {

    titlefile = "P6-L13-Cómo se almacena la energía.pdf";
    name_encode = encodeURI(titlefile);

    content = `<a href='https://files.roboxmaker.com/lessons/T2/${grade}/${name_encode}'>${titlefile}</a>`;

    cmd = `npm run aker-gql-op -- full_lesson_add  --title "${titlefile}"  --summary "" --content "${content}" --author_id "${author_id}"`;
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

//   });
// });

// fs.readFile('./lessons.csv', 'utf8', function (err, data) {
//     var data = data.split(/\r?\n/);
  
//     for(name in data){
//         array = data[name].split(',');

//         class_name = array[0];
//         titlefile = array[1];
//         name_encode = encodeURI(titlefile);

//         lesson_id = array[2];

//         content = `<a href='https://files.aker.network/lessons/U1/${class_name}/${name_encode}.pdf'>${titlefile}</a>`;

//         cmd = `npm run aker-gql-op -- lesson_group_create  --title "${titlefile}"  --summary "" --content "${content}" --group_id ${group_id} --inventory_group_id ${inventory_group_id} --lesson_id ${lesson_id}`;
//         console.log(cmd);

//         //  exec(cmd, (error, stdout, stderr) => {
//         //     if (error) {
//         //         console.log(`error: ${error.message}`);
//         //         return;
//         //     }
//         //     if (stderr) {
//         //         console.log(`stderr: ${stderr}`);
//         //         return;
//         //     }
//         //     console.log(`stdout: ${stdout}`);
//         // });
//     }
// });
