const {
    exec
} = require("child_process");
const fs = require('fs');

let author_id = "ef7451c8-2e7e-4438-99f1-701e3353b768"

data = {
  "folder": "PP",
  "total_lessons": 20,
  "lessons": [
    {
      "id": "PP_L6",
      "title": "Cuenta sus lados y adivina qué es"
    },
  ]
};

data.lessons.forEach(lesson => {
    lessonTitle = lesson.title;

    content = `<div class="raw-html-embed"><iframe style="border:none;" loading="lazy" height="800" width="100%" src="https://files.roboxmaker.com/LESSONS/${data.folder}/Guia_del_maestro_${lesson.id.replace("_", "")}.html"></iframe></div><p>&nbsp;</p>`;

    cmd = `npm run aker-gql-op -- full_lesson_add  --title "${lessonTitle}"  --summary "" --content "${content}" --author_id "${author_id}" --order "${lesson.id.replace("_", "")}" --class_type ${data.folder}`;
    console.log(cmd);

    setTimeout(() => {
        // console.log("Delayed for 1 second.");
    }, "3000")

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
});


