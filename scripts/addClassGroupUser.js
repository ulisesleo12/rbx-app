const {
    exec
} = require("child_process");
const { log } = require("console");
const fs = require('fs');


let class_group_vsa = [
    {
        "class_profile": {
            "name": "Kinder 4-A"
        },
        "group_id": "2b12d7ab-a14c-4de2-b545-f07d3d092519"
    },
    {
        "class_profile": {
            "name": "Kinder 4-B"
        },
        "group_id": "e93f5ad6-459d-460b-8286-8fb95a1e45d1"
    },
    {
        "class_profile": {
            "name": "Kinder 5-A"
        },
        "group_id": "f1b930e0-43cd-4b08-be6c-b08abd192158"
    },
    {
        "class_profile": {
            "name": "Kinder 5-B"
        },
        "group_id": "777b9a29-f5d6-4440-820d-adc9dcf85169"
    },
    {
        "class_profile": {
            "name": "Preparatoria-A"
        },
        "group_id": "0e3ed46b-c10c-43fc-a36e-4522e53282a7"
    },
    {
        "class_profile": {
            "name": "Preparatoria-B"
        },
        "group_id": "f9d503fb-baf7-48a3-92f7-aac7c998546a"
    },
    {
        "class_profile": {
            "name": "Primaria 1-A"
        },
        "group_id": "4e227b92-1b08-486a-854d-919b8460b4ff"
    },
    {
        "class_profile": {
            "name": "Primaria 1-B"
        },
        "group_id": "a747d73a-8dec-4daa-9173-706193fd3f8a"
    },
    {
        "class_profile": {
            "name": "Primaria 2-A"
        },
        "group_id": "a329cd2a-1180-41aa-91fe-824d9efb7731"
    },
    {
        "class_profile": {
            "name": "Primaria 2-B"
        },
        "group_id": "6d1d5bf1-6463-4fdb-b866-c258b385f0e9"
    },
    {
        "class_profile": {
            "name": "Primaria 3-A"
        },
        "group_id": "d94843cf-43cb-4164-a3dd-3319231ac43c"
    },
    {
        "class_profile": {
            "name": "Primaria 3-B"
        },
        "group_id": "72ef0195-d04f-4329-9142-9c2320d1c676"
    },
    {
        "class_profile": {
            "name": "Primaria 4-A"
        },
        "group_id": "4382d399-fb11-4207-9d7c-28bd4c5a2ed1"
    },
    {
        "class_profile": {
            "name": "Primaria 4-B"
        },
        "group_id": "c10c1849-fef2-4871-b804-efe4ee8b1290"
    },
    {
        "class_profile": {
            "name": "Primaria 5-A"
        },
        "group_id": "016767c5-455b-4ec5-ab5b-03c27130ab66"
    },
    {
        "class_profile": {
            "name": "Primaria 5-B"
        },
        "group_id": "c92c0979-50fc-49f3-b2c0-d8b26b802c2d"
    },
    {
        "class_profile": {
            "name": "Primaria 6-A"
        },
        "group_id": "d09bfc87-5097-40b0-b108-6b0a21a979a9"
    },
    {
        "class_profile": {
            "name": "Primaria 6-B"
        },
        "group_id": "1c5da5c6-268e-4f6f-b2ce-5624e954399a"
    },
    {
        "class_profile": {
            "name": "Secundaria 1-A"
        },
        "group_id": "9896a140-64ef-4c5d-9014-a0f9e35a287d"
    },
    {
        "class_profile": {
            "name": "Secundaria 1-B"
        },
        "group_id": "1619a1b9-8c1c-4771-8c7e-64aeafb4a456"
    },
    {
        "class_profile": {
            "name": "Secundaria 2-A"
        },
        "group_id": "267c0d63-5054-4501-8e3c-39de8185cba3"
    },
    {
        "class_profile": {
            "name": "Secundaria 2-B"
        },
        "group_id": "0cb97356-6dec-41a2-b92e-49ff7a93d520"
    },
    {
        "class_profile": {
            "name": "Secundaria 3-A"
        },
        "group_id": "e166e22d-79b2-4c41-9bdd-0a0432e62adb"
    },
    {
        "class_profile": {
            "name": "Secundaria 3-B"
        },
        "group_id": "e1e6e2bd-f5a1-417c-aa2e-78d03b90bd50"
    },
    {
        "class_profile": {
            "name": "Bachillerato 1-A"
        },
        "group_id": "d6e85c19-a5d2-4cd8-bce6-e39f59e2d573"
    },
    {
        "class_profile": {
            "name": "Bachillerato 1-B"
        },
        "group_id": "bbb56eec-e824-48e4-8402-6a0a78e34c43"
    },
    {
        "class_profile": {
            "name": "Bachillerato 2-A"
        },
        "group_id": "c077bc58-293b-4afd-ada7-5a7fac154911"
    },
    {
        "class_profile": {
            "name": "Bachillerato 2-B"
        },
        "group_id": "0e528efc-c287-46ef-a132-6ba0695022ff"
    }
];


let class_group_mca = [
    {
        "class_profile": {
            "name": "Recursos Extra - Arduino"
        },
        "group_id": "bbbf8cdb-db6a-4f57-90c1-d041485daa5a"
    },
    {
        "class_profile": {
            "name": "Recursos Extra - Electrónica"
        },
        "group_id": "dd82c070-4667-4b2e-9b86-e3665225250d"
    },
    {
        "class_profile": {
            "name": "Recursos Extra -  Modelado 3D"
        },
        "group_id": "350b080d-479b-4ed9-a4d4-62898a222723"
    },
    {
        "class_profile": {
            "name": "Recursos Extra - Programación"
        },
        "group_id": "96c8ba02-586b-44eb-8c91-2f8c2036e758"
    },
    // {
    //     "class_profile": {
    //         "name": "Kinder 4-A"
    //     },
    //     "group_id": "74babbf9-6896-4da2-8f59-eaad34b5e944"
    // },
    // {
    //     "class_profile": {
    //         "name": "Kinder 5-A"
    //     },
    //     "group_id": "a68e2df3-f8eb-45c9-9952-10ae2b82098e"
    // },
    {
        "class_profile": {
            "name": "Preparatoria-A"
        },
        "group_id": "56ab7167-15da-4c33-ad32-75578934481b"
    },
    {
        "class_profile": {
            "name": "Primaria 1-A"
        },
        "group_id": "8413523d-78cd-4151-875b-a280a2b6bc3c"
    },
    {
        "class_profile": {
            "name": "Primaria 1-B"
        },
        "group_id": "3b4c4e2d-1487-48ca-9e34-76ef99b9ce85"
    },
    {
        "class_profile": {
            "name": "Primaria 2-A"
        },
        "group_id": "d0d1b10e-1dc6-4205-8b1c-abeb8cb86bdb"
    },
    {
        "class_profile": {
            "name": "Primaria 3-A"
        },
        "group_id": "e9292564-4efc-42dc-ab75-a3dc54fcc5e1"
    },
    {
        "class_profile": {
            "name": "Primaria 4-A"
        },
        "group_id": "5b575ec7-1fbd-44c0-8d75-362c48e85335"
    },
    {
        "class_profile": {
            "name": "Primaria 5-A"
        },
        "group_id": "0850e1e1-6d2e-4690-98ac-2ac8dbc31b45"
    },
    {
        "class_profile": {
            "name": "Primaria 6-A"
        },
        "group_id": "d8a49fb2-80dc-4994-afc1-40384c16a01f"
    },
    {
        "class_profile": {
            "name": "Secundaria 1-A"
        },
        "group_id": "f4c5c7c9-88b3-4d63-94f3-f49b40f797a3"
    },
    {
        "class_profile": {
            "name": "Secundaria 2-A"
        },
        "group_id": "7888833a-fdab-443b-a35c-135e48213b89"
    },
    {
        "class_profile": {
            "name": "Secundaria 3-A"
        },
        "group_id": "026310c2-2d5f-42da-aca6-4fd7d0e58503"
    },
    {
        "class_profile": {
            "name": "Bachillerato 1-A"
        },
        "group_id": "2986cbf5-b78c-4d59-8ef9-fd97cec3242d"
    },
    {
        "class_profile": {
            "name": "Bachillerato 1-B"
        },
        "group_id": "30b90453-43d4-43dd-8555-8be9c2c6aa85"
    },
    {
        "class_profile": {
            "name": "Bachillerato 2-A"
        },
        "group_id": "44286655-8859-4c1a-b7dc-f5da5a59a93e"
    },
    {
        "class_profile": {
            "name": "Bachillerato 2-B"
        },
        "group_id": "6da5d903-518e-47d8-907f-e8a99471b057"
    }
];

const userId = 'fad02612-6758-4d15-aba6-f52848d33dab'; // marielamiron@roboxmaker.com

class_group_mca.forEach(class_group => {
    // console.log(class_group);
    
    let cmd = `npm run aker-gql-op -- add_to_class_group_user  --group_id ${class_group.group_id} --user_id ${userId}`;
    console.log(cmd);

    setTimeout(() => {
        // console.log("Delayed for 1 second.");
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