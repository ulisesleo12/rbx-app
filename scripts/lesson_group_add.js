const {
  exec
} = require("child_process");
const { log } = require("console");
const fs = require('fs');

// let group_id = "64cd9936-89c3-4e39-b963-e82ae8025fc9";

// M3D.forEach(element => {
//   let cmd = `npm run aker-gql-op -- lesson_group_add  --lesson_id ${element[0]} --group_id ${group_id}`;
//   console.log(cmd);

//   exec(cmd, (error, stdout, stderr) => {
//     if (error) {
//       console.log(`error: ${error.message}`);
//       return;
//     }
//     if (stderr) {
//       console.log(`stderr: ${stderr}`);
//       return;
//     }
//     console.log(`stdout: ${stdout}`);
//   });
// });


// let id_class_group = [
//   "b33afa89-6a01-49e0-b01b-b8ae3e434eff", "1c8e2955-4df7-4e29-bbf2-c4e3d35d1932", "a80702b2-b031-488b-813e-6805e6944995", "f7a2248c-6650-46db-a2ee-613fcf81ef32", "8610875d-2479-473a-99e5-ea583d88da2a", "8ba0481a-f90b-4689-b65e-9b0b40450cf9", "05ddb18b-5555-4814-adb6-1326c99aeb77", "c8c5b11a-e86d-4805-b39f-d93787553235", "c7c71f8a-70c9-4272-8f1c-769477ff0845", "8e92bee2-c14b-4b38-a6c3-2a12d10cb33e", "1a71090b-cf15-47f2-8f65-d1f8d7a732a6", "a5527724-ac98-4a3a-bb0d-b778d704c79a", "281ef03d-3fd0-4487-ba6d-9f0c32c659d0", "cde7e66e-b561-497f-8ccf-3933885af7a1", "8205d731-c3e3-4300-a048-944a364c2055", "8176e607-402e-4151-870b-b72cad6ae07c", "3aa280fe-d769-4e97-b67e-6839ca04229a", "8a88a4fd-3f0b-4db6-abdf-a19ab367a4a5"
// ];

// let id_class_group = [
//   "6012f0a0-d1e5-44d9-bbf0-45a4ad9a7525",
//   "7717e433-cb67-45c5-8f64-33154c801d13",
//   "7da1fe6a-f28f-475e-888d-ab6fb308b078",
//   "82a0ef8b-6913-4736-a3b5-315c1d011c60",
//   "8f4d857f-a84a-4287-8222-062583939bca",
//   "dd1fc00e-d473-42bb-85d1-c571993a4260",
//   "ce5de171-e7dc-424e-a34e-c4873ab76cb3",
//   "cfd9058b-216f-4185-8c64-aff99d752b47",
//   "961569ee-98b2-4ab4-a67d-22bcc5114323",
//   "f1a350e6-5f43-455b-a1ce-85dafafa38eb",
//   "6dbf9479-53d6-4e79-b6e5-b4afbf2d8a31",
//   "5c09364f-f8bb-4bd6-a25a-10c6b9d952c2",
//   "1c5244d3-d957-4a2c-9b60-270d48cf2c81"
// ];

// let id_class_group = [
//   "1c8e2955-4df7-4e29-bbf2-c4e3d35d1932", "64cd9936-89c3-4e39-b963-e82ae8025fc9"
// ];
// "64cd9936-89c3-4e39-b963-e82ae8025fc9",
// let id_class_group = [
//   "f1a350e6-5f43-455b-a1ce-85dafafa38eb",
//   "72d0daf5-14f2-4f31-a6e2-0b2d50193fd6",
//   "cfd9058b-216f-4185-8c64-aff99d752b47",
//   "6dbf9479-53d6-4e79-b6e5-b4afbf2d8a31",
//   "961569ee-98b2-4ab4-a67d-22bcc5114323",
//   "662d34bf-e27a-472d-a236-2dea5bc412d6",
//   "4a43badc-a0ec-49d0-b735-c803143f03cd",
//   "a14d7d2a-3e4d-4aaf-bce5-9cecd8192897",
//   "5c09364f-f8bb-4bd6-a25a-10c6b9d952c2",
//   "1c5244d3-d957-4a2c-9b60-270d48cf2c81",
//   "ce5de171-e7dc-424e-a34e-c4873ab76cb3"
// ];
let id_class_group = [
  "a6545ce3-9860-4510-82cc-3649e8e1005d"
];
let id_lesson = [
  "0044855d-4739-4c82-85ff-a9634705e3b9", 
  "08ae1f45-e64f-4361-b7cc-7890a16bfc26", 
  "0aa11903-5cb9-425c-a57b-54c66fdef3ea", 
  "0dbda22e-991a-4617-8336-87417b6752c8", 
  "134163a7-3eec-4266-bb31-c25d089616d8", 
  "16428563-85ff-4b67-a8ca-4427accff9bd", 
  "173868e1-82e3-4f29-9011-fab840996e81", 
  "184bf3a8-492e-40c4-98e5-3208be9a3d17", 
  "20681d83-7aff-4ab4-9662-237207ce5c70", 
  "2980e691-3370-4bc2-b101-3e3b18918ce6", 
  "3976ab47-765c-4adc-bfe4-05e643e3051d", 
  "3ca58210-d5a8-4a4a-ba71-6dffc6e3e504", 
  "40a2cab0-a215-4776-b5a7-861c6ea5a994", 
  "47547945-3d94-485e-b27d-809890cc9519", 
  "4ae171e8-736a-4ca4-8523-fccf0933a261", 
  "50064ae6-d422-43d5-a1f7-10aba1908fac", 
  "5e9e3d61-35a6-4575-8238-00058b92a30c", 
  "62bd7f3f-46a5-4fd1-b7b6-9a8677040a15", 
  "685b3851-b80d-46aa-83df-b5cfd1d90b63", 
  "6a4926ab-71ae-4ecb-aab7-2d02853e6a3a", 
  "6ecc2d7d-3094-47b7-8ae1-5314b8d2c14a", 
  "70c936c1-68fe-4b7b-9c32-4beafe1a6473", 
  "7b1e68a2-2ad1-460d-b8ca-a6e0c09fb9c3", 
  "7cee0c09-a631-4892-9c2a-fcb1149d500b", 
  "802dbb18-366b-4b31-9552-9bed61f39c64", 
  "80607dd8-65a5-4f15-afe0-cd291aee7383", 
  "8912d54d-f3a2-4cfa-8d23-446eeccf2a57", 
  "8f166a29-9447-4fa2-9ec1-0c969511f5e4", 
  "9109984f-d623-4c2f-90cc-65e865d079ca", 
  "919c0ae9-af1f-4c14-8287-932b5607a39c", 
  "92ab6de5-702a-41d9-9320-4f310047fa3e", 
  "98d1a3b9-fef9-4171-a6a1-c1020b0b7262", 
  "99498159-8655-40d4-8f08-20d0953d1551", 
  "9dfa6656-e44c-4940-8f6f-14bab31e95e4", 
  "a18560ae-b21d-4854-a84f-9b21207401cb", 
  "a790013a-cda6-4809-b53f-cfa5c8190a17", 
  "aaacfc4e-b157-4550-8e34-b9fd41f36c59", 
  "ab6e02fe-5d10-4c99-9183-4cc33c17d55b", 
  "abbdf2ea-1787-470d-829d-ae8965dae30f", 
  "ae359cde-51e1-4257-97d2-8483849a52f4", 
  "b456e766-0438-414a-b4e8-2c702b9c6a17", 
  "b5d6bd92-0f56-432a-9892-3f2c1b59d27b", 
  "b75ff8fc-84f2-49e5-8982-a25b7bea6561", 
  "b94c603e-14b9-4b94-814b-c24bb9da11a4", 
  "b98c1589-30e9-49d3-841d-6991eb8631a5", 
  "baa257ce-6fa4-4bd3-84d8-3538cf59c0e7", 
  "c2a2573b-5828-4ed0-a6de-a839b429af7f", 
  "c94be53c-5401-4b02-b06e-b91950f5e273", 
  "cad60531-33bb-479a-a594-8d74ada6cdae", 
  "cbebf62d-bad5-4143-868c-92ee30727995", 
  "d0317cc9-72e9-4c9a-b304-a0bc9caab7cb", 
  "dc00edf2-f0ee-40ef-81b7-19c215b7189d", 
  "dd4c2c46-80ae-4aa6-85f4-92d9a4ef4cba", 
  "e5786887-740a-4235-9942-34a4a5e5f392", 
  "ee486d17-609c-4ba6-b372-93712c33d12f", 
  "f6c2fc99-89e7-497d-bcaa-58177f1d40c8", 
  "f6f727e3-e7b4-4588-92dc-3f80dba87aef", 
  "f980164a-3cc5-4791-a6c9-6354a8d3f9a0", 
  "fb042b52-17ab-4099-81fb-7e07e947a135", 
  "fcfa16f7-bada-4880-af90-50c53553b64c"
];
// let id_lesson = ["f0a1a117-9bac-4b88-9c26-ed06303a4408"];

// <a href='https://files.roboxmaker.com/lessons/T1/Recursos/K4_L14%20Cuento%20-%20%C3%81mbar%20quer%C3%ADa%20conocer%20los%20globos%20aerost%C3%A1ticos.pdf'>K4_L14 Cuento - Ámbar quería conocer los globos aerostáticos.pdf</a>
// <a href='https://files.roboxmaker.com/lessons/T1/Recursos/K4_L3%20Cuento%20-%20Laurita%20y%20sus%20hermanos.pdf'>K4_L3 Cuento - Laurita y sus hermanos.pdf</a>
// <a href='https://files.roboxmaker.com/lessons/T1/Recursos/K5_L11%20Cuento%20-%20%C2%BFPor%20qu%C3%A9%20Leny%20no%20corr%C3%ADa.pdf'>K5_L11 Cuento - ¿Por qué Leny no corría.pdf</a>
// <a href='https://files.roboxmaker.com/lessons/T1/Recursos/P2_L13%20Cuento%20-%20Lucio%20quer%C3%ADa%20ser%20constructor.pdf'>P2_L13 Cuento - Lucio quería ser constructor.pdf</a>
// <a href='https://files.roboxmaker.com/lessons/T1/Recursos/P5_L1%20Cuento%20-%20Los%20castorcitos%20constructores.pdf'>P5_L1 Cuento - Los castorcitos constructores.pdf</a>

new_lessons = [
  // {id: 'abda3144-45ff-4735-8a4e-e9f66cfcef14', section: 'K4_L14', name:'K4_L14 Cuento - Ámbar quería conocer los globos aerostáticos.pdf'},
  // {id: 'c810e665-49b8-464d-bac5-c6907aba88d5', section: 'K4_L3', name:'K4_L3 Cuento - Laurita y sus hermanos.pdf'},
  // {id: '7da22355-c6e5-4984-88b0-4c4c46f06a56', section: 'K5_L11', name:'K5_L11 Cuento - ¿Por qué Leny no corría.pdf'},
  // {id: '4996d6ed-875c-4d85-a293-ab0a8c34607f', section: 'P2_L13', name:'P2_L13 Cuento - Lucio quería ser constructor.pdf'},
  {id: 'c53a51e7-c348-4015-b3f2-8d0ea3c07f1b', section: 'P5_L1', name:'P5_L1 Cuento - Los castorcitos constructores.pdf'}
];

new_groups = [
  // K4 60b70cf7-146e-4636-bbb7-dada9a14dec7
  // // "2b4dce57-5b62-42a1-a82f-5d90f30da7eb",
  // // "2b12d7ab-a14c-4de2-b545-f07d3d092519",
  // // "e93f5ad6-459d-460b-8286-8fb95a1e45d1",
  // // "2a7ccb13-d89b-4987-93d5-7d7669147edf",
  // // "26b7d5ef-58ac-4f08-95b5-db7ff60e636e",
  // // "74babbf9-6896-4da2-8f59-eaad34b5e944",
  // "5016d6e6-4e5e-4ad3-8f00-c4586275ee5f",
  // "7a202d80-42a4-40fb-a2b6-20fd7141f034",
  // "2b4dce57-5b62-42a1-a82f-5d90f30da7eb",
  // "73c4108d-50dc-4b4e-bd1a-8e37e837d151",
  // "2b12d7ab-a14c-4de2-b545-f07d3d092519",
  // "e93f5ad6-459d-460b-8286-8fb95a1e45d1",
  // "9a5e0c04-690f-4d01-8092-246abf6f0cb9",
  // "bc2c75fc-b30a-42b6-b514-85ae99b15e6a",
  // "2a7ccb13-d89b-4987-93d5-7d7669147edf",
  // "999959d9-c418-4254-bd79-b2446babb8e3",
  // "b6b696b1-00c6-4e97-a489-bc2f03dee8d0",
  // "ad807320-8d27-4dcd-8a5d-c4d0102978ef",
  // "dab4c559-5fc8-42db-b2a6-1ac9070ea09a",
  // "e6319799-47c5-4512-b30f-c2214b3c5692",
  // "bf11f394-230c-4d48-a764-bf3ef879598c",
  // "780dedb8-a760-4d40-89d5-7c44b08b7798",
  // "d9934485-12de-4ec3-a3f2-843745542894",
  // "60c68350-900b-4a01-b5ed-ed0811b6ff47",
  // "851b3e06-f0ba-4df6-ba3b-4a5281bcbafc",
  // "8449edc0-2425-4223-b719-83c1de861429",
  // "26b7d5ef-58ac-4f08-95b5-db7ff60e636e",
  // "ba00b982-b4ce-47d4-a6ab-be7513413031",
  // "46016629-1642-4cb3-bade-a6178aa714ac",
  // "b11a72f4-6df2-4abb-8148-5256a7947fa7",
  // "74babbf9-6896-4da2-8f59-eaad34b5e944",
  // "52ef60f8-9111-41e9-a33f-b041200690ec",

  // K5 b064749b-213a-42bc-9d00-61c8d9b94888
  // // "f1b930e0-43cd-4b08-be6c-b08abd192158",
  // // "777b9a29-f5d6-4440-820d-adc9dcf85169",
  // // "dd449a1c-4f0b-4779-be95-7932ee67832a",
  // // "ae8b21df-ec43-4370-9fe6-53b60b889144",
  // // "1edd9be3-561f-4d08-86cd-15c6843de975",
  // // "a68e2df3-f8eb-45c9-9952-10ae2b82098e",
  // "df3521de-152a-40e1-8ae3-3f49d7523269",
  // "f4edaf65-59f8-4441-b69d-9d84ece4bef4",
  // "682d1ffd-2153-4241-8270-d3f5f9f2f605",
  // "c9a6452a-0120-4d2b-8c9d-2b14bc33bb72",
  // "50251cf3-b7e0-43bb-b5d8-c453d1a9c623",
  // "f1b930e0-43cd-4b08-be6c-b08abd192158",
  // "777b9a29-f5d6-4440-820d-adc9dcf85169",
  // "e7554286-f365-4fac-bc89-17e3e97a9b5a",
  // "97ec99a5-7242-4ffb-9992-858ea1b6ada1",
  // "e84e2914-3b12-45cc-a1b1-184da9de6f9b",
  // "9eeb096c-57c9-4546-afb1-9676b683418a",
  // "455edef5-e4c9-4a8a-8f1d-4b850c38ca4d",
  // "2349f66d-95f0-4ae1-880e-02f350d33c0e",
  // "2f3ee2d1-2ce3-419c-bcc3-6b7940c42419",
  // "373b3449-3832-4e01-b873-df549dad8bd9",
  // "e985a4c7-4daf-4eb1-9711-2120637ce017",
  // "7ebc5f66-e62f-430b-a4b9-45375f9a1d58",
  // "8456ec37-49ef-44f9-90b1-1b0a738c64e6",
  // "dd449a1c-4f0b-4779-be95-7932ee67832a",
  // "9076589a-eadb-47cb-a1c8-0f4697900bfe",
  // "49f457d3-c4d6-48fa-8512-066bfc48b58e",
  // "ae8b21df-ec43-4370-9fe6-53b60b889144",
  // "e00423e2-ea8a-4ffd-a9d8-b13a28fe30d9",
  // "1edd9be3-561f-4d08-86cd-15c6843de975",
  // "5e521d22-bda8-42e7-a88f-d030920869e5",
  // "2967273e-b739-469a-a21c-a7c13c32d017",
  // "11e72b29-8071-4cbc-9546-ff115d87509f",
  // "a68e2df3-f8eb-45c9-9952-10ae2b82098e",
  // "b6e1022b-2555-4ceb-b536-c6b4499d3e8b",
  // "870478c9-47a1-4e5b-b1ac-b73cca8e8d17",

  // P2 9f7e4943-3f13-4a58-b1f9-22256a8c09e7
  // // "a329cd2a-1180-41aa-91fe-824d9efb7731",
  // // "6d1d5bf1-6463-4fdb-b866-c258b385f0e9",
  // // "da1081a0-6d97-40c3-97e6-376b6ad56c18",
  // // "641d5f01-f6c8-4da1-80a6-ede4647397a8",
  // // "8db24d62-0b7e-44f1-8ad4-11f3a2c2f5fe",
  // // "73c9e0b1-4cd5-4a1e-993c-5513e5612e35",
  // // "c5500196-8810-474d-bea4-596ef14d4b0a",
  // // "d0d1b10e-1dc6-4205-8b1c-abeb8cb86bdb",
  // "ffa68815-6c8f-4b65-b95e-4a695bd43ac4",
  // "b42a0bd7-4ed8-496c-bb34-0ef79b06a2e2",
  // "67a1b2dd-827f-4b95-9ed9-1a8984cc5816",
  // "b8685dd7-efd8-4367-8b1e-78505adac756",
  // "42a5a822-1259-4997-b97b-520b1e1afb92",
  // "a329cd2a-1180-41aa-91fe-824d9efb7731",
  // "6d1d5bf1-6463-4fdb-b866-c258b385f0e9",
  // "da1081a0-6d97-40c3-97e6-376b6ad56c18",
  // "7aa7c246-d5fa-4bef-af7b-7564988140c3",
  // "9b3c0be4-89da-4da6-952a-bc7c0a8adea2",
  // "6ec55ffc-7ff7-4dfb-8023-0eadc5878167",
  // "0f325020-7acd-4e1e-a1a9-341c4cd3ec98",
  // "9e2c2027-15f8-471e-9682-0db450d41c5e",
  // "6ffb4190-8d24-4d1a-917c-31e2d66f9aae",
  // "f65004e6-cb93-4f7a-a0f6-c07d0e8d35a7",
  // "3eb69bd1-4a9a-47a3-ae57-d613a64dcd4f",
  // "efc7bbc1-71ed-4e97-9657-d5a33954f613",
  // "0e3506e5-e028-4574-96b0-a16634927982",
  // "1f418173-16a4-477d-8bd3-aeca08a8db48",
  // "fced462a-45c9-4040-973b-06f748d822cd",
  // "b755450d-a71d-4af1-9853-0362c44805bf",
  // "641d5f01-f6c8-4da1-80a6-ede4647397a8",
  // "8db24d62-0b7e-44f1-8ad4-11f3a2c2f5fe",
  // "2849349d-55d4-4292-869d-8417f8f824f9",
  // "1c2a6174-e624-4510-8d60-6e449ed20b44",
  // "981d053a-526a-4b09-9213-d483ccee2f4f",
  // "5dffb353-013b-4cad-97c7-bb4d179e4581",
  // "73c9e0b1-4cd5-4a1e-993c-5513e5612e35",
  // "c5500196-8810-474d-bea4-596ef14d4b0a",
  // "4f0cbf9c-e162-41f5-9fc2-d7fd7d0cf5b3",
  // "51be43e2-3df8-4f7b-820f-a43a1df17464",
  // "ba835cf0-7d5d-4c90-b058-09d7d0bb5864",
  // "d0d1b10e-1dc6-4205-8b1c-abeb8cb86bdb",
  // "dbbc2463-3823-4a98-8da8-20e851e87d40",
  // "6cb256ca-3d21-43b4-bb01-55c92c483c34",

  // P5 d795534b-3662-4e91-8c97-273bc52d661e
  // "f5d327cf-921f-4fb3-81c5-ae4a47002ee1",
  // "016767c5-455b-4ec5-ab5b-03c27130ab66",
  // "c92c0979-50fc-49f3-b2c0-d8b26b802c2d",
  // "d3e8f40d-75fe-40e8-a689-efd807b10d93",
  // "ff12b194-87a7-4ea2-8dc6-51a559f00ada",
  // "c58e3cf6-f5ec-4833-a29c-6a1b36e5ee91",
  // "9f70af15-8c0e-4ab8-8244-bfc78714746f",
  // "0850e1e1-6d2e-4690-98ac-2ac8dbc31b45",
  "f5d327cf-921f-4fb3-81c5-ae4a47002ee1",
  "cf6ec161-1b14-4a76-95fb-fe5765456d1f",
  "84f10d3f-d745-4eb3-adef-c0f0c157ed94",
  "016767c5-455b-4ec5-ab5b-03c27130ab66",
  "c92c0979-50fc-49f3-b2c0-d8b26b802c2d",
  "0371e2a9-fc26-4546-b7eb-651f83096873",
  "a7991c68-1169-4e3b-8914-eefed05ac902",
  "4647b439-4c52-49b5-9162-a0d39218342a",
  "e0fe6290-9974-4574-aea2-df721bf461ae",
  "5e1a868e-9481-4ec2-8303-bca509fc6f4e",
  "6a785596-a79b-4006-8cc8-7cd166799522",
  "1b644be1-01bb-48ab-8d84-0f52be42a3e6",
  "3ca7f7f9-0461-4681-8eb3-35b12b8b5611",
  "f46e62b8-aa9e-4562-99d2-01f0df2dd120",
  "5b596147-f96b-427c-a027-a0a95ce7fdaa",
  "11909988-5777-4a06-96bc-a03a6bd371af",
  "4d4cf90e-0e28-4001-ac43-918fbf1e7421",
  "8c198b27-8e24-4831-b55e-a2ef2fae49cb",
  "c4ec55bd-6d7f-40cc-833e-060537108626",
  "d3e8f40d-75fe-40e8-a689-efd807b10d93",
  "ff12b194-87a7-4ea2-8dc6-51a559f00ada",
  "1431e5c3-4e4a-4c56-9131-5cb565b75072",
  "f1ab522e-2ce1-4c36-99e6-fa83539a6877",
  "d066f2c8-c501-4748-af4b-89bd6a4f8870",
  "90f618ac-d0fc-4162-b453-a3ae69a32e47",
  "c58e3cf6-f5ec-4833-a29c-6a1b36e5ee91",
  "69d450a4-6162-4633-818d-1437c9ae65a3",
  "6b099ff2-41bf-417b-b5f8-ef1980e5313a",
  "9f70af15-8c0e-4ab8-8244-bfc78714746f",
  "040f7643-10f3-4635-8256-9e84ab5b6e36",
  "558a52ba-4a19-46a4-b0a3-0ced57d39b9c",
  "0f352b31-3c21-4b9e-9ae4-bec11918de18",
  "0850e1e1-6d2e-4690-98ac-2ac8dbc31b45",
  "8c990c88-9661-43e5-beed-d575e7f2ada1",
];

// PRIMARIA 1 ACTUALIZADO
HP1 = [
  // ELECTRONICA
  {
    "lesson_id": "abbdf2ea-1787-470d-829d-ae8965dae30f",
    "lesson_profile": {
      "title": "EP1-L01-La electricidad"
    }
  },
  {
    "lesson_id": "ee486d17-609c-4ba6-b372-93712c33d12f",
    "lesson_profile": {
      "title": "EP1-L02-Interruptores"
    }
  },
  {
    "lesson_id": "cad60531-33bb-479a-a594-8d74ada6cdae",
    "lesson_profile": {
      "title": "EP1-L03-Luz eléctrica"
    }
  },
  {
    "lesson_id": "919c0ae9-af1f-4c14-8287-932b5607a39c",
    "lesson_profile": {
      "title": "EP1-L04-Sonido"
    }
  },
  {
    "lesson_id": "f6f727e3-e7b4-4588-92dc-3f80dba87aef",
    "lesson_profile": {
      "title": "EP1-L05-Creando viento"
    }
  },
  {
    "lesson_id": "99498159-8655-40d4-8f08-20d0953d1551",
    "lesson_profile": {
      "title": "EP1-L06-Conexión en serie"
    }
  },
  {
    "lesson_id": "70c936c1-68fe-4b7b-9c32-4beafe1a6473",
    "lesson_profile": {
      "title": "EP1-L07-Conexión en paralelo"
    }
  },
  {
    "lesson_id": "3ca58210-d5a8-4a4a-ba71-6dffc6e3e504",
    "lesson_profile": {
      "title": "EP1-L08-Alternando dispositivos"
    }
  },
  {
    "lesson_id": "e5786887-740a-4235-9942-34a4a5e5f392",
    "lesson_profile": {
      "title": "EP1-L09-Movimientos y sonidos"
    }
  },
  {
    "lesson_id": "184bf3a8-492e-40c4-98e5-3208be9a3d17",
    "lesson_profile": {
      "title": "EP1-L10-Luz y oscuridad"
    }
  },
  // Recursos
  {
    "lesson_id": "5d6e75db-29de-423b-a50e-26786f00f095",
    "lesson_profile": {
      "title": "P1-L01-¡Conozcamos nuestras piezas!.pdf"
    }
  },
  {
    "lesson_id": "05f930b8-9a26-453c-b380-b1f590ea5a18",
    "lesson_profile": {
      "title": "P1-L02-Aprendamos a orientar y posicionar objetos.pdf"
    }
  },
  {
    "lesson_id": "e8a3956d-1aa0-4c6a-844c-348662b886cc",
    "lesson_profile": {
      "title": "P1-L03-Construyamos castillos.pdf"
    }
  },
  {
    "lesson_id": "96f1d382-1023-45cc-976e-615d9e707fa4",
    "lesson_profile": {
      "title": "P1-L04-Conozcamos las máquinas simples.pdf"
    }
  },
  {
    "lesson_id": "7e026316-485b-4f3b-8a4c-1cb310cbce7c",
    "lesson_profile": {
      "title": "P1-L05-Conozcamos sobre el plano inclinado.pdf"
    }
  },
  {
    "lesson_id": "f97f0ea4-b463-44b4-8e89-ec16ac63cca3",
    "lesson_profile": {
      "title": "P1-L06-Clasifiquemos y tracemos líneas.pdf"
    }
  },
  {
    "lesson_id": "36328d83-ae69-4372-8bd5-646909a8804d",
    "lesson_profile": {
      "title": "P1-L07-Armemos pistas de carrera.pdf"
    }
  },
  {
    "lesson_id": "2d10e9ba-5235-432e-8110-6593b0fdfddb",
    "lesson_profile": {
      "title": "P1-L08-Aprendamos sobre símbolos patrios.pdf"
    }
  },
  {
    "lesson_id": "62eedd97-f2c7-419a-a774-bc0f8f80e7d2",
    "lesson_profile": {
      "title": "P1-L09-Conozcamos sobre poleas.pdf"
    }
  },
  {
    "lesson_id": "d221d16f-6fc9-4bf4-b24d-87f05ba319b4",
    "lesson_profile": {
      "title": "P1-L10-Juguemos a ser pescadores.pdf"
    }
  },
  {
    "lesson_id": "e25ca98b-c091-4317-aee3-426499b02d82",
    "lesson_profile": {
      "title": "P1-L11-Así funcionan las ruedas.pdf"
    }
  },
  {
    "lesson_id": "f1683df4-a6b9-4767-b170-da7c34f2c1f0",
    "lesson_profile": {
      "title": "P1-L12-Manejemos helicópteros.pdf"
    }
  },
  {
    "lesson_id": "a24bde4f-e214-436a-be12-4115170c2e27",
    "lesson_profile": {
      "title": "P1-L13-Adivina, adivinador.pdf"
    }
  },
  {
    "lesson_id": "32104111-4c7a-4ec4-87ed-8182b6149d87",
    "lesson_profile": {
      "title": "P1-L14-Busquemos semejanzas y diferencias animales.pdf"
    }
  },
  {
    "lesson_id": "ea7cc9ec-f972-41b8-915d-10e73a6d7f53",
    "lesson_profile": {
      "title": "P1-L15-Ideemos y clasifiquemos estructuras.pdf"
    }
  },
  {
    "lesson_id": "e8bcf72b-3b13-40d7-964d-66e3a78fbc8c",
    "lesson_profile": {
      "title": "P1-L16-Cómo funcionan los molinos de viento.pdf"
    }
  },
  {
    "lesson_id": "d590cb62-9295-4afc-97f7-05f12a2fd9f2",
    "lesson_profile": {
      "title": "P1-L17-Encajemos los engranajes.pdf"
    }
  },
  {
    "lesson_id": "26f8772d-fc10-410d-bbec-109c3577bea6",
    "lesson_profile": {
      "title": "P1-L18-Giremos la manivela.pdf"
    }
  },
  {
    "lesson_id": "dec8575a-e892-445b-8c0e-c9743ad9bfa8",
    "lesson_profile": {
      "title": "P1-L19-Describamos nuestro hogar.pdf"
    }
  },
  {
    "lesson_id": "36824c98-f25f-4f93-b18b-0267e8fda137",
    "lesson_profile": {
      "title": "P1-L20-Compitamos en una carrera de carros.pdf"
    }
  },
  {
    "lesson_id": "a9aac3b5-a46f-4a2d-8d21-d500464ea99c",
    "lesson_profile": {
      "title": "P1-L21-El trencito, chucu, chucu, chu….pdf"
    }
  },
  {
    "lesson_id": "557a53eb-da20-42fc-8271-dfbe43ebfb4e",
    "lesson_profile": {
      "title": "P1-L22-Cuánta fuerza tiene el agua.pdf"
    }
  },
  {
    "lesson_id": "72c0b140-d92a-4a9a-b1f3-7605deee4519",
    "lesson_profile": {
      "title": "P1-L23-¡Vamos a la feria!.pdf"
    }
  },
  {
    "lesson_id": "07a08b37-a6ce-4bf3-a02c-bf70a26041bc",
    "lesson_profile": {
      "title": "P1-L24-Cuál es este insecto.pdf"
    }
  },
  {
    "lesson_id": "0426b65d-fcab-4ad2-9678-b0e85a117616",
    "lesson_profile": {
      "title": "P1-L25-Planeemos una competencia de motos.pdf"
    }
  },
  {
    "lesson_id": "15d78d60-1111-4c1a-9bde-afd22f1ae5e1",
    "lesson_profile": {
      "title": "P1-L26-¿Cómo se produce la electricidad.pdf"
    }
  },
  {
    "lesson_id": "df915b7b-5ac4-4c97-919c-34bf7a6a1da8",
    "lesson_profile": {
      "title": "P1-L27-¿Sabes cómo funcionan los interruptores.pdf"
    }
  },
  {
    "lesson_id": "f999edbd-e03e-4150-b84c-f09f92b5b7f9",
    "lesson_profile": {
      "title": "P1-L28-No le temas a la oscuridad.pdf"
    }
  },
  {
    "lesson_id": "23b660ed-8a77-49f4-b240-9cde6ddf4c8b",
    "lesson_profile": {
      "title": "P1-L29-Juguemos con el sonido.pdf"
    }
  },
  {
    "lesson_id": "fa4b3623-3cc5-49d8-9a6b-3f8a8fc5a318",
    "lesson_profile": {
      "title": "P1-L30-¡Hay mucho calor.pdf"
    }
  },
  {
    "lesson_id": "e957e71e-b7e1-4b9d-adcf-17d7d954f1a3",
    "lesson_profile": {
      "title": "P1-L31-Aprendamos nombres propios y comunes en singular y plural.pdf"
    }
  },
  {
    "lesson_id": "d0f86229-8790-47d6-bc4d-b9d98b7d4de2",
    "lesson_profile": {
      "title": "P1-L32-Ocupemos conceptos de tiempo.pdf"
    }
  },
  {
    "lesson_id": "a169b534-4d7d-4f63-bc3b-f9b0689308a5",
    "lesson_profile": {
      "title": "P1-L33-Diversión con circuitos.pdf"
    }
  },
  {
    "lesson_id": "f692901a-63d9-4365-8a22-920bfe1c71f3",
    "lesson_profile": {
      "title": "P1-L34-Detectemos sonidos y objetos.pdf"
    }
  },
  {
    "lesson_id": "e23ef4d9-3a18-4115-9a3f-52af3e993e79",
    "lesson_profile": {
      "title": "P1-L35-Ayudemos a ahorrar energía en casa.pdf"
    }
  }
];

HS1 = [
  {
    "lesson_id": "91436973-1fe2-4432-9d93-2246b51f0b1d",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L01-¿Qué es Arduino?"
    }
  },
  {
    "lesson_id": "0e4f54da-50b7-46c0-8a44-2317d566e90d",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L02-Mi primer programa con Arduino"
    }
  },
  {
    "lesson_id": "b4a32202-8f39-4142-bace-90af91b630fe",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L03-Ciclos repetitivos de programación"
    }
  },
  {
    "lesson_id": "9bc6137d-eac2-41e9-aabb-dd3ea9bc4fa8",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L04-Programando un semáforo"
    }
  },
  {
    "lesson_id": "4e7dba34-1b91-4f01-a857-b22e0ca75f3c",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L05-Principio de las pantallas LED"
    }
  },
  {
    "lesson_id": "c33f99f0-1e84-4e8c-9d22-083d25edb970",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L06-Contador digital"
    }
  },
  {
    "lesson_id": "d8ad7518-1f6c-4bb8-99b5-5ebabb8e4849",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L07-Luxómetro"
    }
  },
  {
    "lesson_id": "0da5a8ba-1108-4731-aca6-b838dfa8b190",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L08-Termómetro digital"
    }
  },
  {
    "lesson_id": "1b7c04d8-a56f-43f5-9663-73ba89fa6dcb",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L09-Movimientos automatizados"
    }
  },
  {
    "lesson_id": "d1e8cc1c-99aa-4339-b22b-a91f4089a386",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "ES1-L10-Robot musical"
    }
  },
  {
    "lesson_id": "57586d37-bb50-4c53-8887-fcc3a9d2487b",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L01-Qué armados puedo realizar.pdf"
    }
  },
  {
    "lesson_id": "2faaf58b-ec9c-4496-9b1e-a0f5c48fd113",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L02-Recordemos el principio de gravedad.pdf"
    }
  },
  {
    "lesson_id": "4988fd55-b4d9-4d33-a468-cdb21112a89d",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L03-Para qué sirven las balanzas.pdf"
    }
  },
  {
    "lesson_id": "11a0e6f5-5706-40a8-a558-b91dc1aa2569",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L04-Aprendamos de máquinas simples.pdf"
    }
  },
  {
    "lesson_id": "f5f5cee7-5fd6-4fbb-adb7-18746354fdf2",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L05-Tiremos la basura en su lugar.pdf"
    }
  },
  {
    "lesson_id": "8f67f488-4c46-4e39-bf4c-23ef8a330570",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L06-Creemos nuestros propios manuales.pdf"
    }
  },
  {
    "lesson_id": "dd284c97-9bd5-4c5d-a3c3-0b240a76af32",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L07-Dejemos que Newton nos enseñe.pdf"
    }
  },
  {
    "lesson_id": "964d913f-a5ab-4014-80d5-cfc7046588a3",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L08-Conozcamos sobre poleas.pdf"
    }
  },
  {
    "lesson_id": "a2da9d3c-5f9e-444d-9450-94fc193e1e92",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L09-Diseñemos con contrapesos.pdf"
    }
  },
  {
    "lesson_id": "c1f44e6b-da92-48c9-8d75-85f1e2280b1c",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L10-Qué necesito para ser constructor.pdf"
    }
  },
  {
    "lesson_id": "99c4b71c-9bd6-401d-9564-24bc0b52e29e",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L11-Aprendamos de mecánica.pdf"
    }
  },
  {
    "lesson_id": "5756c76b-747b-4c43-a569-dc3af105dd78",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L12-Qué hacen los policías.pdf"
    }
  },
  {
    "lesson_id": "7fbd7ff0-63ba-450c-b4f1-dcdb4ad65623",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L13-Así se produce la energía eléctrica.pdf"
    }
  },
  {
    "lesson_id": "d63cbbd0-5cdc-4727-81a3-4ff3069c73b1",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L14-Conozcamos sobre la energía eólica.pdf"
    }
  },
  {
    "lesson_id": "8c718731-d657-41ae-85ba-3124fa53731a",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L15- Ideemos y clasifiquemos estructuras.pdf"
    }
  },
  {
    "lesson_id": "ac9a2fce-6fb0-43da-9300-a7b444cc54aa",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L16-Redactemos noticias.pdf"
    }
  },
  {
    "lesson_id": "ce6ee9e6-697f-4521-b1a8-a2d7bb724cbc",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L17-Cómo funcionan los engranajes.pdf"
    }
  },
  {
    "lesson_id": "003d894f-5540-4e0c-a614-b92c74ec84ff",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L18-Conozcamos sobre energía potencial.pdf"
    }
  },
  {
    "lesson_id": "73704e64-267e-432a-ab10-8b96f7ef76b5",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L19-Por qué se necesita una caja de velocidades.pdf"
    }
  },
  {
    "lesson_id": "b94350ee-1fbc-4754-9a92-b4a4989f675d",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L20-El agua también produce energía.pdf"
    }
  },
  {
    "lesson_id": "c0dd214a-4c8d-4e5c-b4de-a1686295a75d",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L21-Inventemos una película.pdf"
    }
  },
  {
    "lesson_id": "af73cd28-251e-4b60-b747-8cd41de47448",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L22-Todos subamos al tren.pdf"
    }
  },
  {
    "lesson_id": "338aa159-adae-4d3b-85a5-51dc45d40879",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L23-El ciclo de la vida.pdf"
    }
  },
  {
    "lesson_id": "8a602503-3bce-46c5-8c72-f25fb0ceede1",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L24-Mírala alzar el vuelo.pdf"
    }
  },
  {
    "lesson_id": "48c51f31-a1df-423c-ade7-c294d0eb85e4",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L25-Por que se extinguieron los dinosaurios.pdf"
    }
  },
  {
    "lesson_id": "3d6703dc-067a-4f84-a132-cf9eb29e0b83",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L26-¿Qué es Arduino.pdf"
    }
  },
  {
    "lesson_id": "1ca99451-b573-45d7-9ea4-7b32a5d759d1",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L27-Desarrollo mi primer programa con Arduino.pdf"
    }
  },
  {
    "lesson_id": "c84c05d7-c99a-42ac-8021-efdfe090eb9b",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L28-Practico mis conocimientos de programación.pdf"
    }
  },
  {
    "lesson_id": "36f5562d-98c5-46ab-9c9f-06b3c075714d",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L29-Respetemos las leyes de tránsito.pdf"
    }
  },
  {
    "lesson_id": "69912d42-d574-44f7-8ba4-9902fdd729f2",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L30-Programemos y mezclemos luces LED.pdf"
    }
  },
  {
    "lesson_id": "81f88ea8-4ac5-46e9-bc2d-3c8205010512",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L31-Qué son las bibliotecas en software.pdf"
    }
  },
  {
    "lesson_id": "d3c067df-e367-4ce4-8c4c-7507cf647acb",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L32-¿Cuánta luz hay en mi salón de clases.pdf"
    }
  },
  {
    "lesson_id": "ec286840-8187-48b7-a63a-ee3138562399",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L33-Elaboro mi propio termómetro.pdf"
    }
  },
  {
    "lesson_id": "6cf73e48-d304-4e5f-97de-c576741828b3",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L34-Evitemos accidentes de tránsito.pdf"
    }
  },
  {
    "lesson_id": "531b997a-879f-4470-bec8-84891ece8eee",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-L35-Robots complejos.pdf"
    }
  },
  {
    "lesson_id": "44e7aa1d-246c-4550-a591-d2286fecf3bd",
    "group_id": "b5424c4f-d9c9-477d-ae5c-1e0bcd9c70c6",
    "lesson_profile": {
      "title": "S1-RECURSOS ARDUINO.pdf"
    }
  }
];

HS2 = [
  {
      "lesson_id": "a09c1311-74cf-4655-85e9-82c65cecaf12",
      "lesson_profile": {
          "title": "ES2-L01-Programando microcontroladores"
      }
  },
  {
      "lesson_id": "4d1da65b-b254-44e4-a86f-acb7e301ee8a",
      "lesson_profile": {
          "title": "ES2-L02-Comandos básicos Arduino"
      }
  },
  {
      "lesson_id": "2d4bdb16-cca5-405b-9dfb-0fa70d6d62a9",
      "lesson_profile": {
          "title": "ES2-L03-Retardo LED"
      }
  },
  {
      "lesson_id": "dd439a13-f166-4a0f-aee2-fdd50aaca88c",
      "lesson_profile": {
          "title": "ES2-L04-Cuidando el medio ambiente"
      }
  },
  {
      "lesson_id": "e0e17493-91b9-43f7-8600-f360e1f4ded0",
      "lesson_profile": {
          "title": "ES2-L05-Combinando luz LED RGB"
      }
  },
  {
      "lesson_id": "31e00027-dc58-48ca-9caf-c9725ab9dccf",
      "lesson_profile": {
          "title": "ES2-L06-Cronómetro digital"
      }
  },
  {
      "lesson_id": "6238282d-cfa4-47f2-b831-360943d05599",
      "lesson_profile": {
          "title": "ES2-L07-Voltímetro digital"
      }
  },
  {
      "lesson_id": "ff81d490-3391-4739-926e-694fb48bad62",
      "lesson_profile": {
          "title": "ES2-L08-Sonidos en mi habitación"
      }
  },
  {
      "lesson_id": "be436f0a-4f1f-4987-ac0d-3552f39a8dec",
      "lesson_profile": {
          "title": "ES2-L09-Control de velocidad"
      }
  },
  {
      "lesson_id": "5b472be5-f5df-4d27-ab70-708c441897a1",
      "lesson_profile": {
          "title": "ES2-L10-Robot policial"
      }
  },
  {
      "lesson_id": "c7c5942e-a614-4b61-a870-4044caf826b5",
      "lesson_profile": {
          "title": "S2-L01-Identifiquemos ejes de simetría.pdf"
      }
  },
  {
      "lesson_id": "b790cbf7-089f-4c3c-84c3-f3d407c1a058",
      "lesson_profile": {
          "title": "S2-L02-Conozcamos sobre insectos.pdf"
      }
  },
  {
      "lesson_id": "4a4ed5ce-73c0-4276-b0cd-226dea5ed33c",
      "lesson_profile": {
          "title": "S2-L03-Hagamos un torneo de básquetbol.pdf"
      }
  },
  {
      "lesson_id": "0a5c8e02-0eb3-4ef3-a15f-546e20c1c64f",
      "lesson_profile": {
          "title": "S2-L04-Aprendamos sobre la Edad Media.pdf"
      }
  },
  {
      "lesson_id": "a60f6dfe-95cc-4706-8710-789a813174df",
      "lesson_profile": {
          "title": "S2-L05-Primera Ley de Newton.pdf"
      }
  },
  {
      "lesson_id": "cf9a075b-1639-4e8d-9de6-42a1f4449507",
      "lesson_profile": {
          "title": "S2-L06-Para qué sirven los planos inclinados.pdf"
      }
  },
  {
      "lesson_id": "0ae72301-8e1c-4c3a-8a06-09620e43ea79",
      "lesson_profile": {
          "title": "S2-L07-Si cae es porque hay gravedad.pdf"
      }
  },
  {
      "lesson_id": "650c222e-eb75-439e-a9ee-ac634c451740",
      "lesson_profile": {
          "title": "S2-L08-Estas son las poleas.pdf"
      }
  },
  {
      "lesson_id": "bc103673-053b-4383-8955-3e8761fcfef8",
      "lesson_profile": {
          "title": "S2-L09-Aplico los principios de robótica.pdf"
      }
  },
  {
      "lesson_id": "897abcb9-44bd-4e7d-8924-92da71de1e44",
      "lesson_profile": {
          "title": "S2-L10-Prevención en carretera.pdf"
      }
  },
  {
      "lesson_id": "d6a4c318-eb0b-46a7-8a3e-057dd15a3528",
      "lesson_profile": {
          "title": "S2-L11-Principio mecánico de ruedas y ejes.pdf"
      }
  },
  {
      "lesson_id": "ba586cfe-cda3-40c3-8f8a-145778ccd99b",
      "lesson_profile": {
          "title": "S2-L12-Mira cómo se mueve.pdf"
      }
  },
  {
      "lesson_id": "03cc252c-3e51-4ca1-ab24-6267b9d36d46",
      "lesson_profile": {
          "title": "S2-L13-Empuja con fuerza.pdf"
      }
  },
  {
      "lesson_id": "e7327ab6-8e54-4108-b3b1-4660cdbc9618",
      "lesson_profile": {
          "title": "S2-L14-Qué tipo de energía es esta.pdf"
      }
  },
  {
      "lesson_id": "8b16119d-f1b6-4ca6-b74a-3a6537170c1c",
      "lesson_profile": {
          "title": "S2-L15-De dónde vienen los productos.pdf"
      }
  },
  {
      "lesson_id": "ee6821fb-902c-44f5-b37a-8e1ba8d6084a",
      "lesson_profile": {
          "title": "S2-L16-Grandes estructuras.pdf"
      }
  },
  {
      "lesson_id": "867ea02b-c019-4531-8e8a-1cf776f85efb",
      "lesson_profile": {
          "title": "S2-L17-Estos son los engranajes.pdf"
      }
  },
  {
      "lesson_id": "64900f03-3375-4abb-950c-4f4665fbd4ec",
      "lesson_profile": {
          "title": "S2-L18-Ejercitémonos.pdf"
      }
  },
  {
      "lesson_id": "31ab14e7-5215-4d36-8be8-bfc7a63d509c",
      "lesson_profile": {
          "title": "S2-L19-Simple o compuesta.pdf"
      }
  },
  {
      "lesson_id": "c2bb9977-8f2f-446b-a354-31763690d257",
      "lesson_profile": {
          "title": "S2-L20-Cómo ocupamos los combustibles fósiles.pdf"
      }
  },
  {
      "lesson_id": "6f134525-56d8-48bd-98aa-1f84f5bb0406",
      "lesson_profile": {
          "title": "S2-L21-La energía que el agua produce.pdf"
      }
  },
  {
      "lesson_id": "23431db9-2fce-47f4-a05b-a4219e1b004b",
      "lesson_profile": {
          "title": "S2-L22-Desenterremos los fósiles.pdf"
      }
  },
  {
      "lesson_id": "3b731575-949f-4810-8a39-69850108a93b",
      "lesson_profile": {
          "title": "S2-L23-Viajemos al antiguo Egipto.pdf"
      }
  },
  {
      "lesson_id": "2f8ac149-27db-4a3c-ae25-3e4f3ba04275",
      "lesson_profile": {
          "title": "S2-L24-La edad de mi planeta.pdf"
      }
  },
  {
      "lesson_id": "d4a1b8c7-1fc6-4454-b120-6ff59ed21b9d",
      "lesson_profile": {
          "title": "S2-L25-¿Cuál es la edad de la Tierra.pdf"
      }
  },
  {
      "lesson_id": "77a1eeb6-b530-437d-a35b-695026e42c3a",
      "lesson_profile": {
          "title": "S2-L26-Experimentemos con el kit de electrónica.pdf"
      }
  },
  {
      "lesson_id": "18f32c06-ebd3-4128-8b24-6bc88b9e0200",
      "lesson_profile": {
          "title": "S2-L27-Características del software de Arduino.pdf"
      }
  },
  {
      "lesson_id": "26e51b9a-065d-406d-ba5b-218ed4e98980",
      "lesson_profile": {
          "title": "S2-L28-Función de los pulsadores dentro de los circuitos.pdf"
      }
  },
  {
      "lesson_id": "b1837e4b-1341-41e4-8326-f8408a232810",
      "lesson_profile": {
          "title": "S2-L29-Aprendamos a ahorrar energía eléctrica.pdf"
      }
  },
  {
      "lesson_id": "cd127b41-8c7f-4a0a-8e00-dc760f568fae",
      "lesson_profile": {
          "title": "S2-L30-¿Cómo funcionan las luces LED.pdf"
      }
  },
  {
      "lesson_id": "b1307923-5aff-420d-afaf-c5d0886c5104",
      "lesson_profile": {
          "title": "S2-L31-Así se programa con bibliotecas.pdf"
      }
  },
  {
      "lesson_id": "44ac477e-d7e1-4182-b675-bde22a203c06",
      "lesson_profile": {
          "title": "S2-L32-Conozcamos sobre voltímetros y resistencias.pdf"
      }
  },
  {
      "lesson_id": "7662ef8e-6785-4f44-98a0-b89135aa83a4",
      "lesson_profile": {
          "title": "S2-L33-Midamos los sonidos.pdf"
      }
  },
  {
      "lesson_id": "d862a95b-0ab5-4d5a-bc7c-9ddde632c0fa",
      "lesson_profile": {
          "title": "S2-L34-Hagamos una competencia.pdf"
      }
  },
  {
      "lesson_id": "8ab8fa3d-1519-4499-9191-09b60bff6980",
      "lesson_profile": {
          "title": "S2-L35-Tecnología por todos lados.pdf"
      }
  },
  {
      "lesson_id": "1e242516-dd83-4164-96cf-a747c94590ed",
      "lesson_profile": {
          "title": "S2-RECURSOS ARDUINO.pdf"
      }
  }
];

HB1 = [
  {
      "lesson_id": "5df2137a-a997-4e12-955f-c0b32cafdb47",
      "lesson_profile": {
          "title": "B1-L01-Conozcamos sobre mitología grecolatina.pdf"
      }
  },
  {
      "lesson_id": "cc02cb1d-8a10-4376-80e1-6cef25a709a3",
      "lesson_profile": {
          "title": "B1-L02-Maquinaria y modernización.pdf"
      }
  },
  {
      "lesson_id": "06ccf6b3-edee-4a54-8642-8f3ff15d685c",
      "lesson_profile": {
          "title": "B1-L03-Aprendamos de robótica.pdf"
      }
  },
  {
      "lesson_id": "279f45ae-0146-4807-81f3-1238ab0670a0",
      "lesson_profile": {
          "title": "B1-L04-Máquinas simples_La palanca.pdf"
      }
  },
  {
      "lesson_id": "02ea7fe9-262d-43ee-8d99-826585396e65",
      "lesson_profile": {
          "title": "B1-L05-Uso planos inclinados.pdf"
      }
  },
  {
      "lesson_id": "25614f63-83b6-4154-b96e-75e5952945a5",
      "lesson_profile": {
          "title": "B1-L06-Fuerzas paralelas.pdf"
      }
  },
  {
      "lesson_id": "58fc92c0-99a2-45c2-881a-64871d78973f",
      "lesson_profile": {
          "title": "B1-L07-Aprendamos sobre gravedad.pdf"
      }
  },
  {
      "lesson_id": "3a90725e-73f1-4cbe-ac98-4f135bdca1b2",
      "lesson_profile": {
          "title": "B1-L08-Participemos en la competencia.pdf"
      }
  },
  {
      "lesson_id": "52d1a254-fc88-488a-bbb4-a9b8efb084bc",
      "lesson_profile": {
          "title": "B1-L09-Pesos y contrapesos.pdf"
      }
  },
  {
      "lesson_id": "c5d4a07e-321c-43e0-a568-c33fb5c87c0f",
      "lesson_profile": {
          "title": "B1-L10-Armado y uso de poleas.pdf"
      }
  },
  {
      "lesson_id": "f9dca96c-d96f-4a6a-9995-733b0e9e26c3",
      "lesson_profile": {
          "title": "B1-L11-Calculemos la energía.pdf"
      }
  },
  {
      "lesson_id": "3d6aea1e-da1d-4c1a-91e7-4ed21b5e0a38",
      "lesson_profile": {
          "title": "B1-L12-Las velocidades son relativas.pdf"
      }
  },
  {
      "lesson_id": "8f05e4a3-3995-4842-8f3d-a10aeaadd20c",
      "lesson_profile": {
          "title": "B1-L13-Usos de la robótica.pdf"
      }
  },
  {
      "lesson_id": "b237c494-2b3c-4c27-8ba7-e404ee767481",
      "lesson_profile": {
          "title": "B1-L14-La evolución.pdf"
      }
  },
  {
      "lesson_id": "1ff0c8c7-70b0-4287-9c85-d8b449f72c15",
      "lesson_profile": {
          "title": "B1-L15-Estructuras.pdf"
      }
  },
  {
      "lesson_id": "fc8bf396-bdb5-47b9-94cb-eba35414c0ef",
      "lesson_profile": {
          "title": "B1-L16-Estructura de las moléculas.pdf"
      }
  },
  {
      "lesson_id": "25c8efc5-9dbc-43ae-a702-38743937f68c",
      "lesson_profile": {
          "title": "B1-L17-Tipos de engranajes.pdf"
      }
  },
  {
      "lesson_id": "48a491c0-244f-4068-8eff-eab207f0fe88",
      "lesson_profile": {
          "title": "B1-L18-Ensamblo máquinas compuestas.pdf"
      }
  },
  {
      "lesson_id": "b1249cf4-2225-4de1-bfaf-6f90ad4dda84",
      "lesson_profile": {
          "title": "B1-L19-Máquinas de producción.pdf"
      }
  },
  {
      "lesson_id": "85da5746-edda-4b92-b2e6-baf2676c4a37",
      "lesson_profile": {
          "title": "B1-L20-Segunda Ley de Newton.pdf"
      }
  },
  {
      "lesson_id": "a0355923-55ef-4481-8e18-65d5ac828830",
      "lesson_profile": {
          "title": "B1-L21-Esta es la energía hidráulica.pdf"
      }
  },
  {
      "lesson_id": "87e47f87-58e2-4e44-bb87-aed6fa38ed45",
      "lesson_profile": {
          "title": "B1-L22-Una energía amigable con el medio ambiente.pdf"
      }
  },
  {
      "lesson_id": "e7853adc-f765-46bd-beb2-55f4cd4733e2",
      "lesson_profile": {
          "title": "B1-L23-Desarrollo sostenible.pdf"
      }
  },
  {
      "lesson_id": "64417429-c266-4d49-86da-3b6d7326b626",
      "lesson_profile": {
          "title": "B1-L24-Así son los seres vivos.pdf"
      }
  },
  {
      "lesson_id": "741243af-053a-420b-b36d-01817b741ae9",
      "lesson_profile": {
          "title": "B1-L25-Resiliencia.pdf"
      }
  },
  {
      "lesson_id": "cea5f7f4-c38a-4e19-a3b5-3a65cef2b6c2",
      "lesson_profile": {
          "title": "B1-L26-Conoce el kit de electrónica.pdf"
      }
  },
  {
      "lesson_id": "d24fcf06-c712-49b7-95da-91e3f7528158",
      "lesson_profile": {
          "title": "B1-L27-Aprendamos con Arduino.pdf"
      }
  },
  {
      "lesson_id": "c83d46e5-688b-404e-bf76-8904a7f680e8",
      "lesson_profile": {
          "title": "B1-L28-Conozcamos sobre el funcionamiento de los LED.pdf"
      }
  },
  {
      "lesson_id": "5735ee86-ca5b-4fd8-9199-d803481773ef",
      "lesson_profile": {
          "title": "B1-L29-Encontremos usos prácticos a la electrónica.pdf"
      }
  },
  {
      "lesson_id": "e0f046fb-3c38-4a07-ad06-f082a423ca2a",
      "lesson_profile": {
          "title": "B1-L30-Uso bibliotecas de software.pdf"
      }
  },
  {
      "lesson_id": "426ad5e8-3ac9-44be-833f-24e34ca51b24",
      "lesson_profile": {
          "title": "B1-L31-Aplico mis conocimientos de programación.pdf"
      }
  },
  {
      "lesson_id": "cced182b-4668-443f-bdd7-37fbaecce0f5",
      "lesson_profile": {
          "title": "B1-L32-Midamos la resistencia de los materiales.pdf"
      }
  },
  {
      "lesson_id": "3756318d-0541-4acf-9b9f-f0fa34259312",
      "lesson_profile": {
          "title": "B1-L33-Diseñemos un sistema de enfriamiento.pdf"
      }
  },
  {
      "lesson_id": "60b24d04-57a0-4587-97d3-fa2e7c5c7ea7",
      "lesson_profile": {
          "title": "B1-L34-Alerta de aceleración.pdf"
      }
  },
  {
      "lesson_id": "284b1371-f65f-4399-8f96-d21b1f0d378f",
      "lesson_profile": {
          "title": "B1-L35-Programemos un sistema de manejo automático.pdf"
      }
  },
  {
      "lesson_id": "10b857e9-5800-4fc1-95f1-fc5ebe088853",
      "lesson_profile": {
          "title": "B1-RECURSOS ARDUINO.pdf"
      }
  },
  {
      "lesson_id": "8d668666-9f8e-487c-9ff7-e5d5d6ea5531",
      "lesson_profile": {
          "title": "EB1-L01-Fotorresistencia"
      }
  },
  {
      "lesson_id": "93d3c567-25c8-44d2-b163-6f072db1b785",
      "lesson_profile": {
          "title": "EB1-L02-Control LED"
      }
  },
  {
      "lesson_id": "47f91c2b-9c40-447e-9c56-02a11139c6cf",
      "lesson_profile": {
          "title": "EB1-L03-Luces con NeoPixels"
      }
  },
  {
      "lesson_id": "f1a7f157-99ba-44e7-a3c6-9074feba9454",
      "lesson_profile": {
          "title": "EB1-L04-Control vehicular"
      }
  },
  {
      "lesson_id": "7fd69612-88f7-4712-a758-1383aa5cec94",
      "lesson_profile": {
          "title": "EB1-L05-Color musical"
      }
  },
  {
      "lesson_id": "049048f5-3cb5-44a6-8ffd-f878a107d9d7",
      "lesson_profile": {
          "title": "EB1-L06-Temporizador"
      }
  },
  {
      "lesson_id": "d57c52b6-4c57-493a-ab34-e0a4d1cd164c",
      "lesson_profile": {
          "title": "EB1-L07-Ohmímetro"
      }
  },
  {
      "lesson_id": "eef2d120-2366-4ac3-a972-5b12eeb08873",
      "lesson_profile": {
          "title": "EB1-L08-Sistema de enfriamiento"
      }
  },
  {
      "lesson_id": "ec46e47a-049c-4d6e-96f3-d10788a810d7",
      "lesson_profile": {
          "title": "EB1-L09-Luces de aceleración"
      }
  },
  {
      "lesson_id": "d8d5bbd8-9ff1-4a80-889d-6ce09779d566",
      "lesson_profile": {
          "title": "EB1-L10-Viaje seguro"
      }
  }
];

HB2 = [
  {
      "lesson_id": "0eddf47a-6530-4464-8434-2718cb1139dc",
      "lesson_profile": {
          "title": "B2-L01-Organicemos una competencia.pdf"
      }
  },
  {
      "lesson_id": "1a4b98db-13a8-4080-808c-3fc488f79ab0",
      "lesson_profile": {
          "title": "B2-L02-Qué tipo de palanca es esta.pdf"
      }
  },
  {
      "lesson_id": "fbd639b3-b684-4304-b84f-09ab0598c005",
      "lesson_profile": {
          "title": "B2-L03-Apliquemos la Ley de la palanca.pdf"
      }
  },
  {
      "lesson_id": "048b1955-df21-4cbb-9dc9-59faf470971f",
      "lesson_profile": {
          "title": "B2-L04-Conozcamos el funcionamiento de nuestras manos.pdf"
      }
  },
  {
      "lesson_id": "dfc4bab6-b22c-47fd-af62-9c1621eb48aa",
      "lesson_profile": {
          "title": "B2-L05-Tornillo de Arquímedes.pdf"
      }
  },
  {
      "lesson_id": "a1357294-11d5-4497-ad1a-4480709de80a",
      "lesson_profile": {
          "title": "B2-L06-El plano inclinado y la fricción.pdf"
      }
  },
  {
      "lesson_id": "192ee01a-f9d7-4fdf-bf47-1d23e5a18059",
      "lesson_profile": {
          "title": "B2-L07-De una simple a una compuesta.pdf"
      }
  },
  {
      "lesson_id": "ef578c73-53aa-4129-b508-8633f8934058",
      "lesson_profile": {
          "title": "B2-L08-Usos y ventajas de las poleas.pdf"
      }
  },
  {
      "lesson_id": "aa45e389-8371-476e-aca7-dc221481fa68",
      "lesson_profile": {
          "title": "B2-L09-Medios de transporte optativos.pdf"
      }
  },
  {
      "lesson_id": "7100ad8b-1ce1-4f21-bd3b-901146e92c0c",
      "lesson_profile": {
          "title": "B2-L10-Aprendamos de mecánica automotriz.pdf"
      }
  },
  {
      "lesson_id": "d512e270-57f8-4c55-ae8d-67fb21044cfa",
      "lesson_profile": {
          "title": "B2-L11-Argumento sobre nuestras costumbres.pdf"
      }
  },
  {
      "lesson_id": "cb4f2507-cde4-4d7c-826d-0584f062da23",
      "lesson_profile": {
          "title": "B2-L12-Ruedas y ejes.pdf"
      }
  },
  {
      "lesson_id": "3d94f34a-12e3-4123-a22b-0a78fe902f3f",
      "lesson_profile": {
          "title": "B2-L13-Tecnología y desarrollo sostenible.pdf"
      }
  },
  {
      "lesson_id": "a049c017-e5b3-43e7-a488-b545c915ee77",
      "lesson_profile": {
          "title": "B2-L14-Observando ecosistemas imperceptibles.pdf"
      }
  },
  {
      "lesson_id": "f1fb002c-6e0d-4e68-bfa8-e8ec7ed15e86",
      "lesson_profile": {
          "title": "B2-L15-Conozcamos de historia.pdf"
      }
  },
  {
      "lesson_id": "2d9c07ab-8d48-46de-a640-197a7d43b725",
      "lesson_profile": {
          "title": "B2-L16-Usos de las estructuras.pdf"
      }
  },
  {
      "lesson_id": "d01c164a-a9ea-43ca-9067-e0c25c0f1511",
      "lesson_profile": {
          "title": "B2-L17-Conoces los tipos de engranajes.pdf"
      }
  },
  {
      "lesson_id": "84aca061-9b4c-4a0d-8e3d-b6138064d69c",
      "lesson_profile": {
          "title": "B2-L18-Viaje a la luna.pdf"
      }
  },
  {
      "lesson_id": "bb744722-a746-4d11-8e94-689524428db7",
      "lesson_profile": {
          "title": "B2-L19-Las construcciones, ¿perjudican el medio ambiente¿.pdf"
      }
  },
  {
      "lesson_id": "9851e713-e094-4a09-ae63-cccb14387f74",
      "lesson_profile": {
          "title": "B2-L20-Fuerza de fricción.pdf"
      }
  },
  {
      "lesson_id": "1a844c60-b120-4724-b177-335e2cc509fa",
      "lesson_profile": {
          "title": "B2-L21-Energía amigable con el medio ambiente.pdf"
      }
  },
  {
      "lesson_id": "b9a2a33e-4762-4ee4-87f8-d727f75c95f3",
      "lesson_profile": {
          "title": "B2-L22-De la Edad Antigua a la contemporaneidad.pdf"
      }
  },
  {
      "lesson_id": "c8ab53cf-b5e0-41d1-afeb-c81cc743fe9a",
      "lesson_profile": {
          "title": "B2-L23-La edad de la Tierra.pdf"
      }
  },
  {
      "lesson_id": "5636e4a1-8ac7-40c4-b1cd-165453e4a86d",
      "lesson_profile": {
          "title": "B2-L24-La hora mundial.pdf"
      }
  },
  {
      "lesson_id": "bed593d9-534e-4864-83ac-c22d6556f595",
      "lesson_profile": {
          "title": "B2-L25-Somos agentes de publicidad.pdf"
      }
  },
  {
      "lesson_id": "cf9a40fa-56d3-4758-8ed4-b79ea5b7c9cd",
      "lesson_profile": {
          "title": "B2-L26-¿Qué es un relé electromagnético.pdf"
      }
  },
  {
      "lesson_id": "854d5cec-46fa-471a-a0d4-dd939f59a5ae",
      "lesson_profile": {
          "title": "B2-L27-Esto es Arduino.pdf"
      }
  },
  {
      "lesson_id": "d8be861b-2b03-449e-af84-2f789050b7da",
      "lesson_profile": {
          "title": "B2-L28-Programo mis circuitos.pdf"
      }
  },
  {
      "lesson_id": "ebc9223e-6a62-4e7a-a496-b297971c8856",
      "lesson_profile": {
          "title": "B2-L29-¡No grites!.pdf"
      }
  },
  {
      "lesson_id": "f9fe4c6b-be03-44b9-bd33-95d213c51d2f",
      "lesson_profile": {
          "title": "B2-L30-Practico mis conocimientos de programación.pdf"
      }
  },
  {
      "lesson_id": "ce0cf14d-b0ee-4089-b593-bd215a47e1a6",
      "lesson_profile": {
          "title": "B2-L31-Ocupo bibliotecas para programar.pdf"
      }
  },
  {
      "lesson_id": "6de24da5-c4ed-481b-8089-40a74313e070",
      "lesson_profile": {
          "title": "B2-L32-El sistema de numeración binario.pdf"
      }
  },
  {
      "lesson_id": "a7dcb377-6754-4957-b33b-09b6110b0988",
      "lesson_profile": {
          "title": "B2-L33-Qué son las fotorresistencias.pdf"
      }
  },
  {
      "lesson_id": "2d1356f8-f541-4953-8a94-da01629e02a2",
      "lesson_profile": {
          "title": "B2-L34-Identifico el tipo de movimiento.pdf"
      }
  },
  {
      "lesson_id": "09203430-6dbe-4148-9b73-b5366b57f218",
      "lesson_profile": {
          "title": "B2-L35-Características de los robots exploradores.pdf"
      }
  },
  {
      "lesson_id": "7e7f2389-bf9b-4d2a-a9a3-183fd280b6bb",
      "lesson_profile": {
          "title": "B2-RECURSOS ARDUINO.pdf"
      }
  },
  {
      "lesson_id": "a168fc63-35a4-4740-8cf9-06246e7703d0",
      "lesson_profile": {
          "title": "EB2-L01-Relé electromagnético"
      }
  },
  {
      "lesson_id": "fdd5d53a-768c-4a42-812a-b8ff7f3a0846",
      "lesson_profile": {
          "title": "EB2-L02-Luces aceleradas"
      }
  },
  {
      "lesson_id": "55e51b77-26a7-41fa-9983-588f83cf7a8c",
      "lesson_profile": {
          "title": "EB2-L03-Bucles de luces"
      }
  },
  {
      "lesson_id": "a0fd135e-c4ba-4676-a4c2-cee6f4566269",
      "lesson_profile": {
          "title": "EB2-L04-Control de sonido"
      }
  },
  {
      "lesson_id": "2a67911c-34b0-47a3-8a97-a75e730fae02",
      "lesson_profile": {
          "title": "EB2-L05-Música y color"
      }
  },
  {
      "lesson_id": "8c9c794d-38d2-4003-9d48-888e16cb0d58",
      "lesson_profile": {
          "title": "EB2-L06-Programador de tareas"
      }
  },
  {
      "lesson_id": "05f63f77-ce4c-4008-8c5c-782ca67f119c",
      "lesson_profile": {
          "title": "EB2-L07-Calculadora binaria"
      }
  },
  {
      "lesson_id": "f6812f9f-2a2c-48e1-ad01-e2797b351df3",
      "lesson_profile": {
          "title": "EB2-L08-Luces inteligentes"
      }
  },
  {
      "lesson_id": "ad4a8feb-22e3-4bf7-8b6a-786f95490b43",
      "lesson_profile": {
          "title": "EB2-L09-Velocidad y aceleración"
      }
  },
  {
      "lesson_id": "3ce82438-77ea-4f46-b5d4-dd619d702819",
      "lesson_profile": {
          "title": "EB2-L10-Robot explorador"
      }
  }
];

// GROUP_ID = "3b4c4e2d-1487-48ca-9e34-76ef99b9ce85"; // PRIMARIA 1B 28-01-2025

// GROUP_ID = "f09f151c-ec01-4c74-94dc-ec570377b5d6";    //Secundaria 1B 30-01-2025
// GROUP_ID = "d747680e-4aa2-403b-9bb7-35018de5f9fd";    //Secundaria 2B 30-01-2025
// GROUP_ID = "0816afbf-3d86-47b0-aa96-cc5cda4912c5";    //Bachillerato 1B 30-01-2025
// GROUP_ID = "d4d9047c-d2cf-40e6-97f9-71d873a7a337";    //Bachillerato 2B 30-01-2025

// HB2.forEach(lesson => {
//     console.log(lesson.lesson_id);
//       let cmd = `npm run aker-gql-op -- lesson_group_add  --lesson_id ${lesson.lesson_id} --group_id ${GROUP_ID}`;
//       console.log(cmd);

//     setTimeout(() => {
//       // console.log("Delayed for 1 second.");
//     }, "3000")
//     exec(cmd, (error, stdout, stderr) => {
//       if (error) {
//         console.log(`error: ${error.message}`);
//         return;
//       }
//       if (stderr) {
//         console.log(`stderr: ${stderr}`);
//         return;
//       }
//       console.log(`stdout: ${stdout}`);
//     });
// });


new_groups.forEach(group_id => {
  // console.log(group_id);
  new_lessons.forEach(lesson => {
    // console.log(lesson.id);
      let cmd = `npm run aker-gql-op -- lesson_group_add  --lesson_id ${lesson.id} --group_id ${group_id}`;
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
});