const {
    exec
} = require("child_process");


// school_id = 'bdb06dbb-04de-40a4-9a27-60586111f6c0';
school_id = ["a3671320-317f-4798-b1cd-4b4101445ad4"];

// class_ids = [
//     '49dcf3eb-9f56-4a78-848c-d1bc3737296a',
//     'c3debea6-c397-4615-8bad-2ffc02f1d274',
//     '26213bc1-6512-4dd1-a6c1-403e67ab95a4',
//     '2f62253d-385d-49b3-aab4-67f011f9e5c9',
//     'c1ef3b35-d1a3-4cf2-82c7-4804b0b0bc91',
//     '1eba45e4-19aa-4f38-94bc-3bdf809a4923',
//     '22753898-f774-4e13-929f-cfa78ab3cb4b',
//     'd9b21b11-da2f-487c-baeb-b7f2faac7b47',
//     'eeb57591-6c4c-4e1d-98df-b46450f93c69',
//     'de4a6ae6-6e1a-4790-8222-d6ec91f75167',
//     'e3bb5d84-0965-4b4e-b51a-ce9c48316825',
//     '327e8807-2c57-466b-a7ef-a720c0be3982',
//     '2e2f6ad4-0f08-4b97-b2b2-0f7d64387e73',
//     '1ae9a91f-4df8-4794-92ac-8978bb00acd2',
//     '6c47a6e1-44a2-479e-a2fe-76d732db210a',
//     'c7d25e2f-490f-4733-b09c-3a7c198cd1ec',
//     '84c5dc74-316a-42dd-ac70-8b514817ec1f',
//     '45ac0433-348b-471f-8aa5-440b837f2574',
//     'd2958a04-d19e-4f62-a33d-195121d95674',
//     'bbdb6e0f-8d4e-4b4f-bd63-38eb69c8a380',
//     '85d37aa8-b19d-4ed9-ad6d-7292cffe7161',
//     'ae249472-7efc-4c99-9827-bd6426618ac9',
//     '26085ee1-9d71-49d8-aa4b-06931ab0fdc5',
//     '1e3cc74e-73ac-4d08-b3b2-3b492e8ef42d',
//     '516a84c6-3724-4f10-afd4-491911809e90',
//     '5eb0e249-ac91-4542-8525-db7fb8e2e2b4',
//     '28a18041-912d-4e93-b5c4-7cec6a323ac9',
//     'af2c9ac6-0e2d-4ff9-b507-fcc30fa89825',
//     'a51e9765-719b-409d-8fac-336702453bbf',
//     '27ace13c-5082-4c0e-88a0-16e59c5cf50f',
//     '3c8a8810-f074-42a2-8ac0-7049386e54b3',
//     'f187d49d-8f4f-41ca-8320-c28dd1a92079',
//     'afaabeab-ccf4-4bff-8127-d725a30d0ca3',
//     '6c393362-dab4-48d5-9439-764479299207'
// ];
class_ids = [
    // "e3bb5d84-0965-4b4e-b51a-ce9c48316825",
    // "578cd742-e0f4-4ee8-9ccb-3261938ee68e",
    // "1275a328-8ace-48d6-baf8-088c3d0e23ad",
    // "b65c3082-758d-42dd-b78e-09d37964c331",
    // "2b82c966-1b4a-4331-929b-2a39d4e50ebd",
    // "3e45cb58-414d-4f3e-8ade-19d2dac4ea1c",
    // "b177d116-8d52-4ad6-8234-e2dc25b38494",
    // "80c9c92f-674c-464c-b314-4237d69e175a",
    // "4c96ecd9-f91f-4b81-b0fa-06283a0cc9e3",
    // "cef636bc-d44c-46ee-b376-e44cfcb81eed",
    // "091abbc3-79eb-49ea-8f82-66d7dc6d767b",
    // "3bd9d9fd-77a2-432a-8c39-75418b718e50",
    // "e1d31d78-def2-45ef-a1e5-e1008321fb3d",
    // "ae249472-7efc-4c99-9827-bd6426618ac9",
    // "c7dfa83d-3c1e-49d4-a0ac-d06b258c9661",
    // "78701111-8bfd-4229-92ac-1792e7d9e74c",
    // "27ace13c-5082-4c0e-88a0-16e59c5cf50f",
    // "4953df7f-3f4a-4444-be13-7e60bc6c79a2",
    // "1e3cc74e-73ac-4d08-b3b2-3b492e8ef42d",
    // "39810682-3498-4b5a-beb4-1edc25419eb6",
    // "7057f69c-2dc0-4d65-9317-63dce61bb83f",
    // "9e488800-f671-4d0c-acfd-145a120daf3d",
    // "6c47a6e1-44a2-479e-a2fe-76d732db210a",
    // "390724c3-8df0-47ad-bf22-25fab94e8a19",
    // "33c94ae4-80d5-4b30-b3e3-87ff73629d30",
    // "782dcfe5-a8b7-4918-8320-bbe3d3e7791b",
    // "b2e28d81-4ec9-4ffd-abc7-1a34e3d21d99",
    "bd16413a-cac3-47fd-97ce-91249b029d75",
    "86279429-714a-47ef-9744-953f6da77481",
    "4bc6ded5-d1df-471a-95e2-b1539aef4d46",
    "0efc67b9-76bf-48c4-bdab-b71720cc5f1c",
    "577a2392-53a8-4141-ac04-c791de3bf09e",
    "49dcf3eb-9f56-4a78-848c-d1bc3737296a",
    "ae76cb21-b8fb-445a-a2ca-d9d11a96cab5",
    "16af98f2-208d-44d9-97c3-1fa11218ff4a",
    "400a3d55-a369-4ced-81e5-c62ac8e9be78",
    "b17682a9-7350-4741-b9ad-66a937d56522",
    "5eb0e249-ac91-4542-8525-db7fb8e2e2b4",
    "9e0f020e-f58d-4537-a4d5-42c80ac3e2e6",
    "c3debea6-c397-4615-8bad-2ffc02f1d274",
    "45ac0433-348b-471f-8aa5-440b837f2574",
    "9bb3c1f1-cbf4-4d11-a188-c2e205d1896c",
    "bef10eb2-0dd3-45c1-b8e3-66d74bd5f13b",
    "d809f509-20d5-46a7-ae15-35ecc7d77bf3",
    "9613fbbc-4adb-4a37-a519-d924cbeafd29",
    "3381dd33-bfb8-408a-aa57-f1b6e04348ee",
    "3c8a8810-f074-42a2-8ac0-7049386e54b3",
    "82105e9e-567a-43e0-8c13-dbaf7d89120b",
    "fdb87f90-340c-4d0b-a334-ed1ece1d5c3a",
    "54ca4366-24d9-4d29-bc1f-eecf98b61e85",
    "de4a6ae6-6e1a-4790-8222-d6ec91f75167",
    "3bbd3586-c93c-4869-817d-fd9d6af17fe1",
    "c33b498b-991f-4e8e-8d69-0f4c208a8453",
    "f2d8e6d7-81a4-49ea-9150-7f0f61189dd1",
    "b8a27fe9-a340-4783-a78d-8c0bcd143f7a",
    "ed6601d1-06ed-4614-977d-60173465df9a",
    "1ae9a91f-4df8-4794-92ac-8978bb00acd2",
    "2e2f6ad4-0f08-4b97-b2b2-0f7d64387e73",
    "d32d166c-eede-4bfb-a6b8-6c8749c1590c",
    "5fb63030-3221-4140-ad4f-110f67ac599c",
    "af2c9ac6-0e2d-4ff9-b507-fcc30fa89825",
    "9dcf7469-63a9-403d-a304-3fb645974856",
    "5da6c5bd-b214-4e7d-867f-42bb3c01d6b5",
    "5fa0534e-62cf-4a07-8c28-609d4f4e0937",
    "eeb57591-6c4c-4e1d-98df-b46450f93c69",
    "d74218f3-6653-4882-aef2-c638c050f68f",
    "28a18041-912d-4e93-b5c4-7cec6a323ac9",
    "a4c3f7fc-481f-44a1-a827-21b4be004eb9",
    "d740f229-04ca-4a96-b81e-afc224cd4ec1",
    "e64d9140-f7c7-4ea3-a2fa-74aca565bd43",
    "8e3e1fed-c951-4dbd-bc44-3888e5d5f030",
    "db97087c-0271-4469-8d00-319b38117f5f",
    "84c5dc74-316a-42dd-ac70-8b514817ec1f",
    "d2958a04-d19e-4f62-a33d-195121d95674",
    "bbdb6e0f-8d4e-4b4f-bd63-38eb69c8a380",
    "0d830a7a-2512-4e6f-8a2d-20c0d5766d8e",
    "85d37aa8-b19d-4ed9-ad6d-7292cffe7161",
    "26085ee1-9d71-49d8-aa4b-06931ab0fdc5",
    "a51e9765-719b-409d-8fac-336702453bbf",
    "6c393362-dab4-48d5-9439-764479299207",
    "afaabeab-ccf4-4bff-8127-d725a30d0ca3",
    "7a3f1496-3215-4383-950c-3a6953e86853",
    "2fe86f98-aa41-4356-90f4-defa07a09c66",
    "516a84c6-3724-4f10-afd4-491911809e90",
    "fe501a11-9b8a-4a47-a0aa-2ac5515386f1",
    "327e8807-2c57-466b-a7ef-a720c0be3982",
    "f187d49d-8f4f-41ca-8320-c28dd1a92079",
    "c7d25e2f-490f-4733-b09c-3a7c198cd1ec"
];
// console.log(class_ids.length);
// class_ids.forEach(class_id => {
//     school_id.forEach(school_id => {
//         console.log(class_id);
//         // console.log(school_id);
//         let cmd = `npm run aker-gql-op -- class_group_add --class_id ${class_id} --school_id ${school_id}`;
//         console.log(cmd);
//         setTimeout(() => {
//         // console.log("Delayed for 1 second.");
//         }, "3000")
//         exec(cmd, (error, stdout, stderr) => {
//             if (error) {
//                 console.log(`error: ${error.message}`);
//                 return;
//             }
//             if (stderr) {
//                 console.log(`stderr: ${stderr}`);
//                 return;
//             }
//             console.log(`stdout: ${stdout}`);
//         });
//     })
// })

for (i = 0; i < class_ids.length; i++) {
    cmd = `npm run aker-gql-op -- class_group_add --class_id ${class_ids[i]} --school_id ${school_id}`;
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