const {
  exec
} = require("child_process");

class_profile = [{
    "name": "Arduino",
    "class_id": "16af98f2-208d-44d9-97c3-1fa11218ff4a"
  },
  {
    "name": "Electrónica",
    "class_id": "7a3f1496-3215-4383-950c-3a6953e86853"
  },
  {
    "name": "Modelado 3D",
    "class_id": "2fe86f98-aa41-4356-90f4-defa07a09c66"
  },
  {
    "name": "Programación",
    "class_id": "fe501a11-9b8a-4a47-a0aa-2ac5515386f1"
  },
  {
    "name": "K4-Curriculum",
    "class_id": "f2d8e6d7-81a4-49ea-9150-7f0f61189dd1"
  },
  {
    "name": "K4",
    "class_id": "51b80093-a2f4-47cf-84ed-8db9948e8319"
  },
  {
    "name": "K4-A",
    "class_id": "d74218f3-6653-4882-aef2-c638c050f68f"
  },
  {
    "name": "K4-B",
    "class_id": "d809f509-20d5-46a7-ae15-35ecc7d77bf3"
  },
  {
    "name": "K4-C",
    "class_id": "ae76cb21-b8fb-445a-a2ca-d9d11a96cab5"
  },
  {
    "name": "K4-D",
    "class_id": "eeb57591-6c4c-4e1d-98df-b46450f93c69"
  },
  {
    "name": "K4-E",
    "class_id": "de4a6ae6-6e1a-4790-8222-d6ec91f75167"
  },
  {
    "name": "K5-Curriculum",
    "class_id": "e64d9140-f7c7-4ea3-a2fa-74aca565bd43"
  },
  {
    "name": "K5",
    "class_id": "134c17f8-377d-431a-b17d-283e01af98d6"
  },
  {
    "name": "K5-A",
    "class_id": "80c9c92f-674c-464c-b314-4237d69e175a"
  },
  {
    "name": "K5-B",
    "class_id": "d740f229-04ca-4a96-b81e-afc224cd4ec1"
  },
  {
    "name": "K5-C",
    "class_id": "2b82c966-1b4a-4331-929b-2a39d4e50ebd"
  },
  {
    "name": "K5-D",
    "class_id": "e3bb5d84-0965-4b4e-b51a-ce9c48316825"
  },
  {
    "name": "K5-E",
    "class_id": "327e8807-2c57-466b-a7ef-a720c0be3982"
  },
  {
    "name": "PP-Curriculum",
    "class_id": "16af98f2-208d-44d9-97c3-1fa11218ff4a"
  },
  {
    "name": "PP",
    "class_id": "3958b58f-934d-45db-835a-1765c2fa949f"
  },
  {
    "name": "PP-A",
    "class_id": "0efc67b9-76bf-48c4-bdab-b71720cc5f1c"
  },
  {
    "name": "PP-B",
    "class_id": "b17682a9-7350-4741-b9ad-66a937d56522"
  },
  {
    "name": "PP-C",
    "class_id": "577a2392-53a8-4141-ac04-c791de3bf09e"
  },
  {
    "name": "PP-D",
    "class_id": "49dcf3eb-9f56-4a78-848c-d1bc3737296a"
  },
  {
    "name": "PP-E",
    "class_id": "c3debea6-c397-4615-8bad-2ffc02f1d274"
  },
  {
    "name": "P1-Curriculum",
    "class_id": "ed6601d1-06ed-4614-977d-60173465df9a"
  },
  {
    "name": "P1",
    "class_id": "0868f343-9555-4d81-bf77-4b8bc44116c0"
  },
  {
    "name": "P1-A",
    "class_id": "8e3e1fed-c951-4dbd-bc44-3888e5d5f030"
  },
  {
    "name": "P1-B",
    "class_id": "5da6c5bd-b214-4e7d-867f-42bb3c01d6b5"
  },
  {
    "name": "P1-C",
    "class_id": "c33b498b-991f-4e8e-8d69-0f4c208a8453"
  },
  {
    "name": "P1-D",
    "class_id": "84c5dc74-316a-42dd-ac70-8b514817ec1f"
  },
  {
    "name": "P1-E",
    "class_id": "45ac0433-348b-471f-8aa5-440b837f2574"
  },
  {
    "name": "P2-Curriculum",
    "class_id": "0d830a7a-2512-4e6f-8a2d-20c0d5766d8e"
  },
  {
    "name": "P2",
    "class_id": "255d41d0-a8ce-40ba-97d6-76fcd9309385"
  },
  {
    "name": "P2-A",
    "class_id": "3e45cb58-414d-4f3e-8ade-19d2dac4ea1c"
  },
  {
    "name": "P2-B",
    "class_id": "cef636bc-d44c-46ee-b376-e44cfcb81eed"
  },
  {
    "name": "P2-C",
    "class_id": "578cd742-e0f4-4ee8-9ccb-3261938ee68e"
  },
  {
    "name": "P2-D",
    "class_id": "d2958a04-d19e-4f62-a33d-195121d95674"
  },
  {
    "name": "P2-E",
    "class_id": "bbdb6e0f-8d4e-4b4f-bd63-38eb69c8a380"
  },
  {
    "name": "P3-Curriculum",
    "class_id": "091abbc3-79eb-49ea-8f82-66d7dc6d767b"
  },
  {
    "name": "P3",
    "class_id": "7aaa0dc2-f7f6-482c-aa04-d68b126e0244"
  },
  {
    "name": "P3-A",
    "class_id": "c7dfa83d-3c1e-49d4-a0ac-d06b258c9661"
  },
  {
    "name": "P3-B",
    "class_id": "4953df7f-3f4a-4444-be13-7e60bc6c79a2"
  },
  {
    "name": "P3-C",
    "class_id": "e1d31d78-def2-45ef-a1e5-e1008321fb3d"
  },
  {
    "name": "P3-D",
    "class_id": "85d37aa8-b19d-4ed9-ad6d-7292cffe7161"
  },
  {
    "name": "P3-E",
    "class_id": "ae249472-7efc-4c99-9827-bd6426618ac9"
  },
  {
    "name": "P4-Curriculum",
    "class_id": "7057f69c-2dc0-4d65-9317-63dce61bb83f"
  },
  {
    "name": "P4",
    "class_id": "12a4f500-9005-475d-8ee4-880ddd8e3e90"
  },
  {
    "name": "P4-A",
    "class_id": "b2e28d81-4ec9-4ffd-abc7-1a34e3d21d99"
  },
  {
    "name": "P4-B",
    "class_id": "bd16413a-cac3-47fd-97ce-91249b029d75"
  },
  {
    "name": "P4-C",
    "class_id": "782dcfe5-a8b7-4918-8320-bbe3d3e7791b"
  },
  {
    "name": "P4-D",
    "class_id": "26085ee1-9d71-49d8-aa4b-06931ab0fdc5"
  },
  {
    "name": "P4-E",
    "class_id": "1e3cc74e-73ac-4d08-b3b2-3b492e8ef42d"
  },
  {
    "name": "P5-Curriculum",
    "class_id": "bef10eb2-0dd3-45c1-b8e3-66d74bd5f13b"
  },
  {
    "name": "P5",
    "class_id": "f91c864f-16be-4011-bce5-04403440d205"
  },
  {
    "name": "P5-A",
    "class_id": "9e0f020e-f58d-4537-a4d5-42c80ac3e2e6"
  },
  {
    "name": "P5-B",
    "class_id": "9bb3c1f1-cbf4-4d11-a188-c2e205d1896c"
  },
  {
    "name": "P5-C",
    "class_id": "4bc6ded5-d1df-471a-95e2-b1539aef4d46"
  },
  {
    "name": "P5-D",
    "class_id": "516a84c6-3724-4f10-afd4-491911809e90"
  },
  {
    "name": "P5-E",
    "class_id": "5eb0e249-ac91-4542-8525-db7fb8e2e2b4"
  },
  {
    "name": "P6-Curriculum",
    "class_id": "9613fbbc-4adb-4a37-a519-d924cbeafd29"
  },
  {
    "name": "P6",
    "class_id": "6c8f103e-0e1c-4a80-b394-2fd74476d9dc"
  },
  {
    "name": "P6-A",
    "class_id": "d32d166c-eede-4bfb-a6b8-6c8749c1590c"
  },
  {
    "name": "P6-B",
    "class_id": "3381dd33-bfb8-408a-aa57-f1b6e04348ee"
  },
  {
    "name": "P6-C",
    "class_id": "1275a328-8ace-48d6-baf8-088c3d0e23ad"
  },
  {
    "name": "P6-D",
    "class_id": "28a18041-912d-4e93-b5c4-7cec6a323ac9"
  },
  {
    "name": "P6-E",
    "class_id": "af2c9ac6-0e2d-4ff9-b507-fcc30fa89825"
  },
  {
    "name": "S1-Curriculum",
    "class_id": "78701111-8bfd-4229-92ac-1792e7d9e74c"
  },
  {
    "name": "S1",
    "class_id": "86b47b24-5cff-4381-a5d3-b5c5de646049"
  },
  {
    "name": "S1-A",
    "class_id": "4c96ecd9-f91f-4b81-b0fa-06283a0cc9e3"
  },
  {
    "name": "S1-B",
    "class_id": "3bd9d9fd-77a2-432a-8c39-75418b718e50"
  },
  {
    "name": "S1-C",
    "class_id": "9e488800-f671-4d0c-acfd-145a120daf3d"
  },
  {
    "name": "S1-D",
    "class_id": "a51e9765-719b-409d-8fac-336702453bbf"
  },
  {
    "name": "S1-E",
    "class_id": "27ace13c-5082-4c0e-88a0-16e59c5cf50f"
  },
  {
    "name": "S2-Curriculum",
    "class_id": "86279429-714a-47ef-9744-953f6da77481"
  },
  {
    "name": "S2",
    "class_id": "e1fdf9c7-796d-4cc1-97fd-c850d0bddf06"
  },
  {
    "name": "S2-A",
    "class_id": "400a3d55-a369-4ced-81e5-c62ac8e9be78"
  },
  {
    "name": "S2-B",
    "class_id": "33c94ae4-80d5-4b30-b3e3-87ff73629d30"
  },
  {
    "name": "S2-C",
    "class_id": "82105e9e-567a-43e0-8c13-dbaf7d89120b"
  },
  {
    "name": "S2-D",
    "class_id": "3c8a8810-f074-42a2-8ac0-7049386e54b3"
  },
  {
    "name": "S2-E",
    "class_id": "f187d49d-8f4f-41ca-8320-c28dd1a92079"
  },
  {
    "name": "S3-Curriculum",
    "class_id": "3bbd3586-c93c-4869-817d-fd9d6af17fe1"
  },
  {
    "name": "S3",
    "class_id": "2360ff2c-c123-47fe-ab6c-a024d249ff89"
  },
  {
    "name": "S3-A",
    "class_id": "54ca4366-24d9-4d29-bc1f-eecf98b61e85"
  },
  {
    "name": "S3-B",
    "class_id": "fdb87f90-340c-4d0b-a334-ed1ece1d5c3a"
  },
  {
    "name": "S3-C",
    "class_id": "b8a27fe9-a340-4783-a78d-8c0bcd143f7a"
  },
  {
    "name": "S3-D",
    "class_id": "afaabeab-ccf4-4bff-8127-d725a30d0ca3"
  },
  {
    "name": "S3-E",
    "class_id": "6c393362-dab4-48d5-9439-764479299207"
  },
  {
    "name": "B1-Curriculum",
    "class_id": "a4c3f7fc-481f-44a1-a827-21b4be004eb9"
  },
  {
    "name": "B1",
    "class_id": "c96db3e5-79b3-465a-bce1-5bd65fca5cbb"
  },
  {
    "name": "B1-A",
    "class_id": "9dcf7469-63a9-403d-a304-3fb645974856"
  },
  {
    "name": "B1-B",
    "class_id": "5fa0534e-62cf-4a07-8c28-609d4f4e0937"
  },
  {
    "name": "B1-C",
    "class_id": "5fb63030-3221-4140-ad4f-110f67ac599c"
  },
  {
    "name": "B1-D",
    "class_id": "2e2f6ad4-0f08-4b97-b2b2-0f7d64387e73"
  },
  {
    "name": "B1-E",
    "class_id": "1ae9a91f-4df8-4794-92ac-8978bb00acd2"
  },
  {
    "name": "B2-Curriculum",
    "class_id": "db97087c-0271-4469-8d00-319b38117f5f"
  },
  {
    "name": "B2",
    "class_id": "edcf882f-0bfa-4aa8-a2b3-3ce1caeae579"
  },
  {
    "name": "B2-A",
    "class_id": "b177d116-8d52-4ad6-8234-e2dc25b38494"
  },
  {
    "name": "B2-B",
    "class_id": "b65c3082-758d-42dd-b78e-09d37964c331"
  },
  {
    "name": "B2-C",
    "class_id": "39810682-3498-4b5a-beb4-1edc25419eb6"
  },
  {
    "name": "B2-D",
    "class_id": "6c47a6e1-44a2-479e-a2fe-76d732db210a"
  },
  {
    "name": "B2-E",
    "class_id": "c7d25e2f-490f-4733-b09c-3a7c198cd1ec"
  },
]


order = 100;
class_profile.forEach(element => {

  // console.log(element);
  let cmd = `npm run aker-gql-op -- order_class_update  --class_id ${element['class_id']} --order ${order}`;
  console.log(cmd);

  // exec(cmd, (error, stdout, stderr) => {
  //           if (error) {
  //                 console.log(`error: ${error.message}`);
  //                 return;
  //             }
  //             if (stderr) {
  //                   console.log(`stderr: ${stderr}`);
  //                   return;
  //               }
  //               console.log(`stdout: ${stdout}`);
  //           });
  console.log(order);
  order += 100;
});