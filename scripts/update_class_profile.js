

const {
        exec
    } = require("child_process");
    const fs = require('fs');


extra_class_id = [
    "390724c3-8df0-47ad-bf22-25fab94e8a19", // Recursos Extra - Arduino
    "7a3f1496-3215-4383-950c-3a6953e86853", // Recursos Extra - Electrónica
    "2fe86f98-aa41-4356-90f4-defa07a09c66", // Recursos Extra -  Modelado 3D
    "fe501a11-9b8a-4a47-a0aa-2ac5515386f1", // Recursos Extra - Programación
];

kinder_4_class_id = [
    "d74218f3-6653-4882-aef2-c638c050f68f", // Kinder 4-A
    "d809f509-20d5-46a7-ae15-35ecc7d77bf3", // Kinder 4-B
    "ae76cb21-b8fb-445a-a2ca-d9d11a96cab5", // Kinder 4-C
    "eeb57591-6c4c-4e1d-98df-b46450f93c69", // Kinder 4-D
    "de4a6ae6-6e1a-4790-8222-d6ec91f75167"  // Kinder 4-E
];

kinder_5_class_id = [
    "80c9c92f-674c-464c-b314-4237d69e175a", // Kinder 5-A
    "d740f229-04ca-4a96-b81e-afc224cd4ec1", // Kinder 5-B
    "2b82c966-1b4a-4331-929b-2a39d4e50ebd", // Kinder 5-C
    "e3bb5d84-0965-4b4e-b51a-ce9c48316825", // Kinder 5-D
    "327e8807-2c57-466b-a7ef-a720c0be3982"  // Kinder 5-E
];

preparatory_class_id = [
    "0efc67b9-76bf-48c4-bdab-b71720cc5f1c", // Preparatory A
    "b17682a9-7350-4741-b9ad-66a937d56522", // Preparatory B
    "577a2392-53a8-4141-ac04-c791de3bf09e", // Preparatory C
    "49dcf3eb-9f56-4a78-848c-d1bc3737296a", // Preparatory D
    "c3debea6-c397-4615-8bad-2ffc02f1d274"  // Preparatory E
];

primary_1_class_id = [
    "8e3e1fed-c951-4dbd-bc44-3888e5d5f030", // Primary 1-A
    "5da6c5bd-b214-4e7d-867f-42bb3c01d6b5", // Primary 1-B
    "c33b498b-991f-4e8e-8d69-0f4c208a8453", // Primary 1-C
    "84c5dc74-316a-42dd-ac70-8b514817ec1f", // Primary 1-D
    "45ac0433-348b-471f-8aa5-440b837f2574"  // Primary 1-E
];

primary_2_class_id = [
    "3e45cb58-414d-4f3e-8ade-19d2dac4ea1c", // Primary 2-A
    "cef636bc-d44c-46ee-b376-e44cfcb81eed", // Primary 2-B
    "578cd742-e0f4-4ee8-9ccb-3261938ee68e", // Primary 2-C
    "d2958a04-d19e-4f62-a33d-195121d95674", // Primary 2-D
    "bbdb6e0f-8d4e-4b4f-bd63-38eb69c8a380"  // Primary 2-E
];

primary_3_class_id = [
    "c7dfa83d-3c1e-49d4-a0ac-d06b258c9661", // Primary 3-A
    "4953df7f-3f4a-4444-be13-7e60bc6c79a2", // Primary 3-B
    "e1d31d78-def2-45ef-a1e5-e1008321fb3d", // Primary 3-C
    "85d37aa8-b19d-4ed9-ad6d-7292cffe7161", // Primary 3-D
    "ae249472-7efc-4c99-9827-bd6426618ac9"  // Primary 3-E
];

primary_4_class_id = [
    "b2e28d81-4ec9-4ffd-abc7-1a34e3d21d99", // Primary 4-A
    "bd16413a-cac3-47fd-97ce-91249b029d75", // Primary 4-B
    "782dcfe5-a8b7-4918-8320-bbe3d3e7791b", // Primary 4-C
    "26085ee1-9d71-49d8-aa4b-06931ab0fdc5", // Primary 4-D
    "1e3cc74e-73ac-4d08-b3b2-3b492e8ef42d"  // Primary 4-E
];

primary_5_class_id = [
    "9e0f020e-f58d-4537-a4d5-42c80ac3e2e6", // Primary 5-A
    "9bb3c1f1-cbf4-4d11-a188-c2e205d1896c", // Primary 5-B
    "4bc6ded5-d1df-471a-95e2-b1539aef4d46", // Primary 5-C
    "516a84c6-3724-4f10-afd4-491911809e90", // Primary 5-D
    "5eb0e249-ac91-4542-8525-db7fb8e2e2b4"  // Primary 5-E
];

primary_6_class_id = [
    "d32d166c-eede-4bfb-a6b8-6c8749c1590c", // Primary 6-A
    "3381dd33-bfb8-408a-aa57-f1b6e04348ee", // Primary 6-B
    "1275a328-8ace-48d6-baf8-088c3d0e23ad", // Primary 6-C
    "28a18041-912d-4e93-b5c4-7cec6a323ac9", // Primary 6-D
    "af2c9ac6-0e2d-4ff9-b507-fcc30fa89825"  // Primary 6-E
];

secondary_1_class_id = [
    "4c96ecd9-f91f-4b81-b0fa-06283a0cc9e3", // Secondary 1-A
    "3bd9d9fd-77a2-432a-8c39-75418b718e50", // Secondary 1-B
    "9e488800-f671-4d0c-acfd-145a120daf3d", // Secondary 1-C
    "a51e9765-719b-409d-8fac-336702453bbf", // Secondary 1-D
    "27ace13c-5082-4c0e-88a0-16e59c5cf50f"  // Secondary 1-E
];

secondary_2_class_id = [
    "400a3d55-a369-4ced-81e5-c62ac8e9be78", // Secondary 2-A
    "33c94ae4-80d5-4b30-b3e3-87ff73629d30", // Secondary 2-B
    "82105e9e-567a-43e0-8c13-dbaf7d89120b", // Secondary 2-C
    "3c8a8810-f074-42a2-8ac0-7049386e54b3", // Secondary 2-D
    "f187d49d-8f4f-41ca-8320-c28dd1a92079"  // Secondary 2-E
];

secondary_3_class_id = [
    "54ca4366-24d9-4d29-bc1f-eecf98b61e85", // Secondary 3-A
    "fdb87f90-340c-4d0b-a334-ed1ece1d5c3a", // Secondary 3-B
    "b8a27fe9-a340-4783-a78d-8c0bcd143f7a", // Secondary 3-C
    "afaabeab-ccf4-4bff-8127-d725a30d0ca3", // Secondary 3-D
    "6c393362-dab4-48d5-9439-764479299207"  // Secondary 3-E
];

bash_1_class_id = [
    "9dcf7469-63a9-403d-a304-3fb645974856", // Bash 1-A
    "5fa0534e-62cf-4a07-8c28-609d4f4e0937", // Bash 1-B
    "5fb63030-3221-4140-ad4f-110f67ac599c", // Bash 1-C
    "2e2f6ad4-0f08-4b97-b2b2-0f7d64387e73", // Bash 1-D
    "1ae9a91f-4df8-4794-92ac-8978bb00acd2"  // Bash 1-E
];

bash_2_class_id = [
    "b177d116-8d52-4ad6-8234-e2dc25b38494", // Bash 2-A
    "b65c3082-758d-42dd-b78e-09d37964c331", // Bash 2-B
    "39810682-3498-4b5a-beb4-1edc25419eb6", // Bash 2-C
    "6c47a6e1-44a2-479e-a2fe-76d732db210a" // Bash 2-D
];

bash_3_class_id = [
    "f0dc05cb-8855-431b-b9d8-e40d9245011d", // Bash 3-A
    "735ff4be-cbbc-4ed0-9f5a-6a2507372abd", // Bash 3-B
    "56f896ed-edcc-489f-a960-d2dfb74ee866", // Bash 3-C
    "43ba107f-9280-49e8-83cc-0f2a438bb6af", // Bash 3-D
    "3228b3fe-859d-47df-91d7-49f9c717296d"  // Bash 3-E
];


section_ids = [
    // "61a644fd-1f51-4486-bdf3-75b550aee5ad",    // Recursos Extra - Arduino
    "59276ef4-1593-4657-90a2-6c2e1da2c7d0",    // Recursos Extra - Electronica
    // "b998dd87-76ef-4779-958a-fdeeaa409b76",    // Recursos Extra - Modelado 3D
    // "41c7262c-31f6-4ca5-a18b-5aa5b2fa144e",    // Recursos Extra - Programacion
    // "60b70cf7-146e-4636-bbb7-dada9a14dec7",    // Kinder 4
    // "b064749b-213a-42bc-9d00-61c8d9b94888",    // Kinder 5
    // "bb2cb615-f962-4006-8909-0aa76b5cb6a0",    // Preparatoria
    // "2fbe0f72-2640-49dc-8956-053c41e22ced",    // Primaria 1
    // "9f7e4943-3f13-4a58-b1f9-22256a8c09e7",    // Primaria 2
    // "de08dd9d-1837-4126-910b-564fcbc37af7",    // Primaria 3
    // "9b19d943-824d-4ce6-8109-b58f63267ce8",    // Primaria 4
    // "d795534b-3662-4e91-8c97-273bc52d661e",    // Primaria 5
    // "87532805-371e-4487-aa16-4073608e150f",    // Primaria 6
    // "5ec69d3c-47f4-4402-a72a-8c6343d895ad",    // Secundaria 1
    // "90048ec5-5bdb-4035-a6f6-e2a8fee823bc",    // Secundaria 2
    // "ed063e4c-d6e3-4d78-879b-eff403110ee4",    // Secundaria 3
    // "d4250f2a-1772-4eb9-b7ce-fb846697a117",    // Bachillerato 1
    // "91881526-4621-45b8-b4de-f2a8342fd9c9",    // Bachillerato 2
    // "2a4d8490-c450-4d2a-8a45-701bd5f2fc76"     // Bachillerato 3
];


bash_3_class_id.forEach(class_id => {
    // console.log(group_id);
    section_ids.forEach(section_id => {
        // console.log(lesson_id)
        let cmd = `npm run aker-gql-op -- update_class_profile  --class_id ${class_id} --section_id ${section_id}`;
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
    })
})