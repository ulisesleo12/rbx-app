const { exec } = require("child_process");

class_ids = [
    {
        "name": "Recursos Extra - Arduino",
        "class_id": "390724c3-8df0-47ad-bf22-25fab94e8a19"
    },
    {
        "name": "Recursos Extra - Electrónica",
        "class_id": "7a3f1496-3215-4383-950c-3a6953e86853"
    },
    {
        "name": "Recursos Extra -  Modelado 3D",
        "class_id": "2fe86f98-aa41-4356-90f4-defa07a09c66"
    },
    {
        "name": "Recursos Extra - Programación",
        "class_id": "fe501a11-9b8a-4a47-a0aa-2ac5515386f1"
    },
    {
        "name": "Kinder 4-A",
        "class_id": "d74218f3-6653-4882-aef2-c638c050f68f"
    },
    // {
    //     "name": "Kinder 4-B",
    //     "class_id": "d809f509-20d5-46a7-ae15-35ecc7d77bf3"
    // },
    // {
    //     "name": "Kinder 4-C",
    //     "class_id": "ae76cb21-b8fb-445a-a2ca-d9d11a96cab5"
    // },
    // {
    //     "name": "Kinder 4-D",
    //     "class_id": "eeb57591-6c4c-4e1d-98df-b46450f93c69"
    // },
    // {
    //     "name": "Kinder 4-E",
    //     "class_id": "de4a6ae6-6e1a-4790-8222-d6ec91f75167"
    // },
    {
        "name": "Kinder 5-A",
        "class_id": "80c9c92f-674c-464c-b314-4237d69e175a"
    },
    // {
    //     "name": "Kinder 5-B",
    //     "class_id": "d740f229-04ca-4a96-b81e-afc224cd4ec1"
    // },
    // {
    //     "name": "Kinder 5-C",
    //     "class_id": "2b82c966-1b4a-4331-929b-2a39d4e50ebd"
    // },
    // {
    //     "name": "Kinder 5-D",
    //     "class_id": "e3bb5d84-0965-4b4e-b51a-ce9c48316825"
    // },
    // {
    //     "name": "Kinder 5-E",
    //     "class_id": "327e8807-2c57-466b-a7ef-a720c0be3982"
    // },
    {
        "name": "Preparatoria-A",
        "class_id": "0efc67b9-76bf-48c4-bdab-b71720cc5f1c"
    },
    // {
    //     "name": "Preparatoria-B",
    //     "class_id": "b17682a9-7350-4741-b9ad-66a937d56522"
    // },
    // {
    //     "name": "Preparatoria-C",
    //     "class_id": "577a2392-53a8-4141-ac04-c791de3bf09e"
    // },
    // {
    //     "name": "Preparatoria-D",
    //     "class_id": "49dcf3eb-9f56-4a78-848c-d1bc3737296a"
    // },
    // {
    //     "name": "Preparatoria-E",
    //     "class_id": "c3debea6-c397-4615-8bad-2ffc02f1d274"
    // },
    {
        "name": "Primaria 1-A",
        "class_id": "8e3e1fed-c951-4dbd-bc44-3888e5d5f030"
    },
    // {
    //     "name": "Primaria 1-B",
    //     "class_id": "5da6c5bd-b214-4e7d-867f-42bb3c01d6b5"
    // },
    // {
    //     "name": "Primaria 1-C",
    //     "class_id": "c33b498b-991f-4e8e-8d69-0f4c208a8453"
    // },
    // {
    //     "name": "Primaria 1-D",
    //     "class_id": "84c5dc74-316a-42dd-ac70-8b514817ec1f"
    // },
    // {
    //     "name": "Primaria 1-E",
    //     "class_id": "45ac0433-348b-471f-8aa5-440b837f2574"
    // },
    {
        "name": "Primaria 2-A",
        "class_id": "3e45cb58-414d-4f3e-8ade-19d2dac4ea1c"
    },
    // {
    //     "name": "Primaria 2-B",
    //     "class_id": "cef636bc-d44c-46ee-b376-e44cfcb81eed"
    // },
    // {
    //     "name": "Primaria 2-C",
    //     "class_id": "578cd742-e0f4-4ee8-9ccb-3261938ee68e"
    // },
    // {
    //     "name": "Primaria 2-D",
    //     "class_id": "d2958a04-d19e-4f62-a33d-195121d95674"
    // },
    // {
    //     "name": "Primaria 2-E",
    //     "class_id": "bbdb6e0f-8d4e-4b4f-bd63-38eb69c8a380"
    // },
    {
        "name": "Primaria 3-A",
        "class_id": "c7dfa83d-3c1e-49d4-a0ac-d06b258c9661"
    },
    // {
    //     "name": "Primaria 3-B",
    //     "class_id": "4953df7f-3f4a-4444-be13-7e60bc6c79a2"
    // },
    // {
    //     "name": "Primaria 3-C",
    //     "class_id": "e1d31d78-def2-45ef-a1e5-e1008321fb3d"
    // },
    // {
    //     "name": "Primaria 3-D",
    //     "class_id": "85d37aa8-b19d-4ed9-ad6d-7292cffe7161"
    // },
    // {
    //     "name": "Primaria 3-E",
    //     "class_id": "ae249472-7efc-4c99-9827-bd6426618ac9"
    // },
    {
        "name": "Primaria 4-A",
        "class_id": "b2e28d81-4ec9-4ffd-abc7-1a34e3d21d99"
    },
    // {
    //     "name": "Primaria 4-B",
    //     "class_id": "bd16413a-cac3-47fd-97ce-91249b029d75"
    // },
    // {
    //     "name": "Primaria 4-C",
    //     "class_id": "782dcfe5-a8b7-4918-8320-bbe3d3e7791b"
    // },
    // {
    //     "name": "Primaria 4-D",
    //     "class_id": "26085ee1-9d71-49d8-aa4b-06931ab0fdc5"
    // },
    // {
    //     "name": "Primaria 4-E",
    //     "class_id": "1e3cc74e-73ac-4d08-b3b2-3b492e8ef42d"
    // },
    {
        "name": "Primaria 5-A",
        "class_id": "9e0f020e-f58d-4537-a4d5-42c80ac3e2e6"
    },
    // {
    //     "name": "Primaria 5-B",
    //     "class_id": "9bb3c1f1-cbf4-4d11-a188-c2e205d1896c"
    // },
    // {
    //     "name": "Primaria 5-C",
    //     "class_id": "4bc6ded5-d1df-471a-95e2-b1539aef4d46"
    // },
    // {
    //     "name": "Primaria 5-D",
    //     "class_id": "516a84c6-3724-4f10-afd4-491911809e90"
    // },
    // {
    //     "name": "Primaria 5-E",
    //     "class_id": "5eb0e249-ac91-4542-8525-db7fb8e2e2b4"
    // },
    {
        "name": "Primaria 6-A",
        "class_id": "d32d166c-eede-4bfb-a6b8-6c8749c1590c"
    },
    // {
    //     "name": "Primaria 6-B",
    //     "class_id": "3381dd33-bfb8-408a-aa57-f1b6e04348ee"
    // },
    // {
    //     "name": "Primaria 6-C",
    //     "class_id": "1275a328-8ace-48d6-baf8-088c3d0e23ad"
    // },
    // {
    //     "name": "Primaria 6-D",
    //     "class_id": "28a18041-912d-4e93-b5c4-7cec6a323ac9"
    // },
    // {
    //     "name": "Primaria 6-E",
    //     "class_id": "af2c9ac6-0e2d-4ff9-b507-fcc30fa89825"
    // },
    {
        "name": "Secundaria 1-A",
        "class_id": "4c96ecd9-f91f-4b81-b0fa-06283a0cc9e3"
    },
    {
        "name": "Secundaria 1-B",
        "class_id": "3bd9d9fd-77a2-432a-8c39-75418b718e50"
    },
    // {
    //     "name": "Secundaria 1-C",
    //     "class_id": "9e488800-f671-4d0c-acfd-145a120daf3d"
    // },
    // {
    //     "name": "Secundaria 1-D",
    //     "class_id": "a51e9765-719b-409d-8fac-336702453bbf"
    // },
    // {
    //     "name": "Secundaria 1-E",
    //     "class_id": "27ace13c-5082-4c0e-88a0-16e59c5cf50f"
    // },
    {
        "name": "Secundaria 2-A",
        "class_id": "400a3d55-a369-4ced-81e5-c62ac8e9be78"
    },
    {
        "name": "Secundaria 2-B",
        "class_id": "33c94ae4-80d5-4b30-b3e3-87ff73629d30"
    },
    // {
    //     "name": "Secundaria 2-C",
    //     "class_id": "82105e9e-567a-43e0-8c13-dbaf7d89120b"
    // },
    // {
    //     "name": "Secundaria 2-D",
    //     "class_id": "3c8a8810-f074-42a2-8ac0-7049386e54b3"
    // },
    // {
    //     "name": "Secundaria 2-E",
    //     "class_id": "f187d49d-8f4f-41ca-8320-c28dd1a92079"
    // },
    {
        "name": "Secundaria 3-A",
        "class_id": "54ca4366-24d9-4d29-bc1f-eecf98b61e85"
    },
    {
        "name": "Secundaria 3-B",
        "class_id": "fdb87f90-340c-4d0b-a334-ed1ece1d5c3a"
    },
    // {
    //     "name": "Secundaria 3-C",
    //     "class_id": "b8a27fe9-a340-4783-a78d-8c0bcd143f7a"
    // },
    // {
    //     "name": "Secundaria 3-D",
    //     "class_id": "afaabeab-ccf4-4bff-8127-d725a30d0ca3"
    // },
    // {
    //     "name": "Secundaria 3-E",
    //     "class_id": "6c393362-dab4-48d5-9439-764479299207"
    // },
    {
        "name": "Bachillerato 1-A",
        "class_id": "9dcf7469-63a9-403d-a304-3fb645974856"
    },
    {
        "name": "Bachillerato 1-B",
        "class_id": "5fa0534e-62cf-4a07-8c28-609d4f4e0937"
    },
    // {
    //     "name": "Bachillerato 1-C",
    //     "class_id": "5fb63030-3221-4140-ad4f-110f67ac599c"
    // },
    // {
    //     "name": "Bachillerato 1-D",
    //     "class_id": "2e2f6ad4-0f08-4b97-b2b2-0f7d64387e73"
    // },
    // {
    //     "name": "Bachillerato 1-E",
    //     "class_id": "1ae9a91f-4df8-4794-92ac-8978bb00acd2"
    // },
    {
        "name": "Bachillerato 2-A",
        "class_id": "b177d116-8d52-4ad6-8234-e2dc25b38494"
    },
    {
        "name": "Bachillerato 2-B",
        "class_id": "b65c3082-758d-42dd-b78e-09d37964c331"
    },
    // {
    //     "name": "Bachillerato 2-C",
    //     "class_id": "39810682-3498-4b5a-beb4-1edc25419eb6"
    // },
    // {
    //     "name": "Bachillerato 2-D",
    //     "class_id": "6c47a6e1-44a2-479e-a2fe-76d732db210a"
    // },
    // {
    //     "name": "Bachillerato 3-A",
    //     "class_id": "f0dc05cb-8855-431b-b9d8-e40d9245011d"
    // },
    // {
    //     "name": "Bachillerato 3-B",
    //     "class_id": "735ff4be-cbbc-4ed0-9f5a-6a2507372abd"
    // },
    // {
    //     "name": "Bachillerato 3-C",
    //     "class_id": "56f896ed-edcc-489f-a960-d2dfb74ee866"
    // },
    // {
    //     "name": "Bachillerato 3-D",
    //     "class_id": "43ba107f-9280-49e8-83cc-0f2a438bb6af"
    // },
    // {
    //     "name": "Bachillerato 3-E",
    //     "class_id": "3228b3fe-859d-47df-91d7-49f9c717296d"
    // }
];

school_id = "2382e193-c4fa-4120-8615-23fb9d980514";

for (i = 0; i < class_ids.length; i++) {
    cmd = `npm run aker-gql-op -- class_group_add --class_id ${class_ids[i].class_id} --school_id ${school_id}`;
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
}