const {exec} = require("child_process");
const fs = require('fs');

let grade = "K5"

const dir = `../../LECCIONES/${grade}`;
let author_id = "ef7451c8-2e7e-4438-99f1-701e3353b768"

new_lessons = [
    {section: 'K4_L14', name:'K4_L14 Cuento - Ámbar quería conocer los globos aerostáticos.pdf'},
    {section: 'K4_L3', name:'K4_L3 Cuento - Laurita y sus hermanos.pdf'},
    {section: 'K5_L11', name:'K5_L11 Cuento - ¿Por qué Leny no corría.pdf'},
    {section: 'P2_L13', name:'P2_L13 Cuento - Lucio quería ser constructor.pdf'},
    {section: 'P5_L1', name:'P5_L1 Cuento - Los castorcitos constructores.pdf'}
];

new_lessons.forEach(lesson => {
    name_encode = encodeURI(lesson.name);
    // console.log(name_encode);

    content = `<a href='https://files.roboxmaker.com/lessons/T1/Recursos/${name_encode}'>${lesson.name}</a>`;

    console.log(content);
});

// fs.readdir(dir, (err, files) => {
//   files.forEach(file => {

    // titlefile = "P6-L13-Cómo se almacena la energía.pdf";
    // name_encode = encodeURI(titlefile);

    // content = `<a href='https://files.roboxmaker.com/lessons/T2/${grade}/${name_encode}'>${titlefile}</a>`;

    // cmd = `npm run aker-gql-op -- full_lesson_add  --title "${titlefile}"  --summary "" --content "${content}" --author_id "${author_id}"`;
    // console.log(cmd);

    //  exec(cmd, (error, stdout, stderr) => {
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
