const {
    exec
} = require("child_process");
const fs = require('fs');


data = {
  "folder": "Secundaria1",
  "total_lessons": 26,
  "lessons": [
    {
      "variables": {
        "title": "¿Qué armados puedo realizar?",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L1.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "106b3710-a92f-4e52-9378-b7ec2b164448",
        "lesson_code": "S1L1"
      }
    },
    {
      "variables": {
        "title": "Recordemos el principio de gravedad",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L2.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "e63449a6-d6ff-42f4-bcec-8bfd4dc499a4",
        "lesson_code": "S1L2"
      }
    },
    {
      "variables": {
        "title": "¿Para qué sirven las balanzas?",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L3.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "b6173b12-9d0b-410f-adc3-a3f6b943909d",
        "lesson_code": "S1L3"
      }
    },
    {
      "variables": {
        "title": "Aprendamos de máquinas simples",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L4.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "3e0124f9-766f-4955-a816-459178567d08",
        "lesson_code": "S1L4"
      }
    },
    {
      "variables": {
        "title": "Tiremos la basura en su lugar",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L5.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "e9893403-34e4-4852-8fc1-bb7a5ae84e00",
        "lesson_code": "S1L5"
      }
    },
    {
      "variables": {
        "title": "Creemos nuestros propios manuales",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L6.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "347134bc-5bcf-4a0e-8198-aa87d93593d3",
        "lesson_code": "S1L6"
      }
    },
    {
      "variables": {
        "title": "Dejemos que Newton nos enseñe.",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L7.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "eb95c7dd-1e72-4137-a666-f8bae0a10884",
        "lesson_code": "S1L7"
      }
    },
    {
      "variables": {
        "title": "Conozcamos sobre poleas",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L8.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "7190ba7b-2057-444b-bf1b-b3aa1db39ad9",
        "lesson_code": "S1L8"
      }
    },
    {
      "variables": {
        "title": "Diseñemos con contrapesos",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L9.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "ca5b75fc-c95f-451b-8ff5-f27e2abbc587",
        "lesson_code": "S1L9"
      }
    },
    {
      "variables": {
        "title": "¿Qué necesito para ser constructor?",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L10.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "90fa476c-1018-46ce-8ec3-ebbb09feee9f",
        "lesson_code": "S1L10"
      }
    },
    {
      "variables": {
        "title": "Aprendamos de mecánica",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L11.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "0f8bc20c-cf08-4abe-aacf-c3f790d34920",
        "lesson_code": "S1L11"
      }
    },
    {
      "variables": {
        "title": "¿Qué hacen los policías?",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L12.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "a437d15b-aabe-425b-a0a0-7bc3631387a8",
        "lesson_code": "S1L12"
      }
    },
    {
      "variables": {
        "title": "Así se obtiene la energía eléctrica",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L13.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "2c3b3a61-dde4-4e22-a654-2c82d3b3c826",
        "lesson_code": "S1L13"
      }
    },
    {
      "variables": {
        "title": "Conozcamos sobre la energía eólica",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L14.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "cd121794-1c98-4070-86af-b5263a868d38",
        "lesson_code": "S1L14"
      }
    },
    {
      "variables": {
        "title": "Ideemos y clasifiquemos estructuras",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L15.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "88c1e0ea-5c24-4ba8-977b-aea8de8e8061",
        "lesson_code": "S1L15"
      }
    },
    {
      "variables": {
        "title": "Redactemos noticias",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L16.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "cc91cc28-c5b4-4572-b6dd-af4d6e2c7d7f",
        "lesson_code": "S1L16"
      }
    },
    {
      "variables": {
        "title": "Cómo funcionan los engranajes",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L17.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "ec07c6cd-301f-4a70-a4d7-46b8ad3eee2f",
        "lesson_code": "S1L17"
      }
    },
    {
      "variables": {
        "title": "Conozcamos sobre energía potencial",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L18.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "3e9008d1-405b-477c-bb05-a3390afba46d",
        "lesson_code": "S1L18"
      }
    },
    {
      "variables": {
        "title": "¿Por qué se necesita una caja de velocidades?",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L19.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "fe2db1c6-7191-4679-b11c-0c8f1dc12996",
        "lesson_code": "S1L19"
      }
    },
    {
      "variables": {
        "title": "El agua también es fuente de energía",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L20.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "78e186fa-0012-497c-b3c2-51c3ca2feb40",
        "lesson_code": "S1L20"
      }
    },
    {
      "variables": {
        "title": "Inventemos una película",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L21.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "33cf11a3-4342-41ce-a8c2-174a0bdfb730",
        "lesson_code": "S1L21"
      }
    },
    {
      "variables": {
        "title": "Todos subamos al tren",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L22.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "51e3ef00-8bf7-4af7-b783-b39a9afe8379",
        "lesson_code": "S1L22"
      }
    },
    {
      "variables": {
        "title": "El ciclo de la vida",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L23.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "53a3fa6f-8dd6-4139-961e-a723eaf83990",
        "lesson_code": "S1L23"
      }
    },
    {
      "variables": {
        "title": "Mírala alzar el vuelo",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L24.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "f0bace31-807e-4fac-ba5e-c0e4686b867d",
        "lesson_code": "S1L24"
      }
    },
    {
      "variables": {
        "title": "¿Por qué se extinguieron los dinosaurios?",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L25.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "58209068-96c7-4a6d-bec6-5ceafbb5e7a2",
        "lesson_code": "S1L25"
      }
    },
    {
      "variables": {
        "title": "Identifiquemos ejes de simetría",
        "content": "<div class=raw-html-embed><iframe style=border:none; loading=lazy height=800 width=100% src=https://files.roboxmaker.com/LESSONS/Secundaria1/Guia_del_maestro_S1L26.html></iframe></div><p>&nbsp;</p>"
      },
      "response": {
        "lesson_id": "411a1ac3-1fec-410b-af0c-93203f75e786",
        "lesson_code": "S1L26"
      }
    }
  ]
}


data.lessons.forEach((lesson, index) => {
    let lesson_id = lesson.response.lesson_id;
    let order = lesson.response.lesson_code

    let cmd = `npm run aker-gql-op -- UpdateLessonOrder  --lesson_id "${lesson_id}" --order "${order}"`;
    console.log(cmd);
    
    setTimeout(() => {
        console.log("Delayed for 1 second.");
    }, "3000")

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
});
