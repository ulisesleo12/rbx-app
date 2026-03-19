const {
    exec
} = require("child_process");
const fs = require('fs');

/*
SCHOOLS INVENTORY GROUP ID

AC: "64cd9936-89c3-4e39-b963-e82ae8025fc9"
VSA: "f1a350e6-5f43-455b-a1ce-85dafafa38eb"

ICO: "cfd9058b-216f-4185-8c64-aff99d752b47"
MSA: "6dbf9479-53d6-4e79-b6e5-b4afbf2d8a31"
CIO: "961569ee-98b2-4ab4-a67d-22bcc5114323"
CBM: "5c09364f-f8bb-4bd6-a25a-10c6b9d952c2"
NSP: "1c5244d3-d957-4a2c-9b60-270d48cf2c81"
LBI: "ce5de171-e7dc-424e-a34e-c4873ab76cb3"
*/

HP1 = [
    {
        "robot_id": "9d19d2de-2a7a-4b6e-8fe8-03fc4ab7194a",
        "robot_profile": {
            "name": "P1_L01_1"
        }
    },
    {
        "robot_id": "4829889d-a336-4f06-a4f9-193e8615f770",
        "robot_profile": {
            "name": "P1_L01_2"
        }
    },
    {
        "robot_id": "23540a80-b423-4bc5-9c85-2b7c07d0d4a9",
        "robot_profile": {
            "name": "P1_L01_3"
        }
    },
    {
        "robot_id": "0d067e34-12dd-48fc-beaa-21a89a57eda7",
        "robot_profile": {
            "name": "P1_L02_1"
        }
    },
    {
        "robot_id": "18a449ef-9aa4-4802-9b4e-1122ec606733",
        "robot_profile": {
            "name": "P1_L02_2"
        }
    },
    {
        "robot_id": "1caf787e-1609-48af-8a6c-fa8230deb5d7",
        "robot_profile": {
            "name": "P1_L03"
        }
    },
    {
        "robot_id": "9949878d-8db1-470b-912f-0284312dd7e7",
        "robot_profile": {
            "name": "P1_L04"
        }
    },
    {
        "robot_id": "a0d66726-1f64-4f6d-96b5-c01494351028",
        "robot_profile": {
            "name": "P1_L05"
        }
    },
    {
        "robot_id": "9604718f-1544-40d3-babb-e18b268b7427",
        "robot_profile": {
            "name": "P1_L06"
        }
    },
    {
        "robot_id": "3a7e6686-f1a0-48d1-b874-c180e8f945c4",
        "robot_profile": {
            "name": "P1_L07"
        }
    },
    {
        "robot_id": "321d541b-3665-46f0-94fb-3125674ed7b1",
        "robot_profile": {
            "name": "P1_L08"
        }
    },
    {
        "robot_id": "4ca80a3f-0c6e-492f-92ca-efc5f6bb86af",
        "robot_profile": {
            "name": "P1_L09"
        }
    },
    {
        "robot_id": "d7469224-ba1d-4719-a9a7-64176bece1fd",
        "robot_profile": {
            "name": "P1_L10"
        }
    },
    {
        "robot_id": "31be9911-1caa-45ba-a1ce-c90173c1ec44",
        "robot_profile": {
            "name": "P1_L11"
        }
    },
    {
        "robot_id": "cf58bb24-fef1-4fb7-b735-d9d02613e09d",
        "robot_profile": {
            "name": "P1_L12"
        }
    },
    {
        "robot_id": "60976579-c19c-443a-b9a5-51eb2b548543",
        "robot_profile": {
            "name": "P1_L13"
        }
    },
    {
        "robot_id": "baf82f4d-a27f-44b4-98ff-df77a6533ff7",
        "robot_profile": {
            "name": "P1_L14_1"
        }
    },
    {
        "robot_id": "2f707d83-0399-46da-8436-5bd67eb57572",
        "robot_profile": {
            "name": "P1_L14_2"
        }
    },
    {
        "robot_id": "d7735a12-4141-4ab3-ab75-7c4dac74ed65",
        "robot_profile": {
            "name": "P1_L15"
        }
    },
    {
        "robot_id": "719a8f98-668b-4b19-86d9-43857d28ff82",
        "robot_profile": {
            "name": "P1_L16"
        }
    },
    {
        "robot_id": "c6455f55-ae33-474e-8398-5b1c0a52a50f",
        "robot_profile": {
            "name": "P1_L17"
        }
    },
    {
        "robot_id": "6b273a27-97d3-41af-88ff-6b4d86c76349",
        "robot_profile": {
            "name": "P1_L18"
        }
    },
    {
        "robot_id": "2f164c37-159d-4b29-8356-52eb348019b0",
        "robot_profile": {
            "name": "P1_L19"
        }
    },
    {
        "robot_id": "003d65f9-ff3f-47ea-bc54-fff553a84454",
        "robot_profile": {
            "name": "P1_L20"
        }
    },
    {
        "robot_id": "e1cfb397-aa05-41fb-bc9b-2c9416499c07",
        "robot_profile": {
            "name": "P1_L21"
        }
    },
    {
        "robot_id": "55e3f756-410c-49d8-acb4-42466b83eebd",
        "robot_profile": {
            "name": "P1_L22"
        }
    },
    {
        "robot_id": "a514cd55-1cb4-4cdd-8b3f-c9e0d2b9514d",
        "robot_profile": {
            "name": "P1_L23"
        }
    },
    {
        "robot_id": "37841fe3-900a-4e9a-bfe6-b78d17c29abb",
        "robot_profile": {
            "name": "P1_L24"
        }
    },
    {
        "robot_id": "2e01052b-ba84-46bd-a259-254336d2fc66",
        "robot_profile": {
            "name": "P1_L25"
        }
    }
];

HS1 = [
    {
        "robot_id": "ea632f20-b399-43bf-a9b0-5da88eb12081",
        "robot_profile": {
            "name": "S1_L01"
        }
    },
    {
        "robot_id": "ee09e175-1f41-4930-8b27-cb0064d007e4",
        "robot_profile": {
            "name": "S1_L02"
        }
    },
    {
        "robot_id": "f42b55f2-6e39-4c06-80ed-e54097856013",
        "robot_profile": {
            "name": "S1_L03"
        }
    },
    {
        "robot_id": "d3c9896e-7993-4671-a72d-d0965ba9f554",
        "robot_profile": {
            "name": "S1_L04"
        }
    },
    {
        "robot_id": "f9ca3d20-327e-44a4-a99e-2ebd29195619",
        "robot_profile": {
            "name": "S1_L05"
        }
    },
    {
        "robot_id": "5274fe6b-f812-4a3c-9099-e5aed9d92e92",
        "robot_profile": {
            "name": "S1_L06"
        }
    },
    {
        "robot_id": "e4ae7b8e-4cc1-4e85-b742-dc395358ad51",
        "robot_profile": {
            "name": "S1_L07"
        }
    },
    {
        "robot_id": "16e8f8a0-647a-4700-b07d-fd40e6a14920",
        "robot_profile": {
            "name": "S1_L08"
        }
    },
    {
        "robot_id": "2cbf06ef-4ddb-4e88-a6a3-1055631edf0c",
        "robot_profile": {
            "name": "S1_L09"
        }
    },
    {
        "robot_id": "c4fe1d53-9061-443d-bcb2-51f5bf64d3ec",
        "robot_profile": {
            "name": "S1_L10"
        }
    },
    {
        "robot_id": "bef5ccc5-5a2d-4bd9-a46b-1b404182a4c5",
        "robot_profile": {
            "name": "S1_L11"
        }
    },
    {
        "robot_id": "b7cebb69-a0f7-4c0c-98df-bb6a148cc463",
        "robot_profile": {
            "name": "S1_L12"
        }
    },
    {
        "robot_id": "092ecb10-124c-433c-87d7-7c385955d487",
        "robot_profile": {
            "name": "S1_L13"
        }
    },
    {
        "robot_id": "674b3d5c-e89b-4d5c-a247-4a7410eab830",
        "robot_profile": {
            "name": "S1_L14"
        }
    },
    {
        "robot_id": "2fe35712-9250-49c8-a058-f133e63e85a7",
        "robot_profile": {
            "name": "S1_L15"
        }
    },
    {
        "robot_id": "f15d76b0-b35d-4fa2-9f85-6de523ace5cd",
        "robot_profile": {
            "name": "S1_L16"
        }
    },
    {
        "robot_id": "49b46e88-bd84-46dd-b604-5d4d650469bb",
        "robot_profile": {
            "name": "S1_L17"
        }
    },
    {
        "robot_id": "4b1aa74f-5456-46ef-8dcf-5b6c448f8bf7",
        "robot_profile": {
            "name": "S1_L18"
        }
    },
    {
        "robot_id": "c04c82e5-3b15-4f13-a8c5-3a1f5bfed874",
        "robot_profile": {
            "name": "S1_L19"
        }
    },
    {
        "robot_id": "0d5906bb-1006-404e-b537-6b55add27c4b",
        "robot_profile": {
            "name": "S1_L20"
        }
    },
    {
        "robot_id": "86fc4324-716a-4339-8b3c-7ac941acbff9",
        "robot_profile": {
            "name": "S1_L21"
        }
    },
    {
        "robot_id": "d41d9ce2-cccf-4583-9341-68b9d45509a0",
        "robot_profile": {
            "name": "S1_L22"
        }
    },
    {
        "robot_id": "dc532d9e-8955-4555-ae74-94dec1b4841b",
        "robot_profile": {
            "name": "S1_L23"
        }
    },
    {
        "robot_id": "23e0d350-35e9-4cfa-9514-49e085dca784",
        "robot_profile": {
            "name": "S1_L24"
        }
    },
    {
        "robot_id": "1d6d2c47-8356-4b4b-aecd-05e612b161a7",
        "robot_profile": {
            "name": "S1_L25"
        }
    }
];

HS2 = [
    {
        "robot_id": "2e16c488-db99-41a7-84d7-38c70873ad19",
        "robot_profile": {
            "name": "S2_L01"
        }
    },
    {
        "robot_id": "b0a8f874-d4fb-4090-849a-55447c79d085",
        "robot_profile": {
            "name": "S2_L02"
        }
    },
    {
        "robot_id": "2466c274-33f7-433b-855a-aeea3fd16928",
        "robot_profile": {
            "name": "S2_L03"
        }
    },
    {
        "robot_id": "2b335e3e-179e-469c-89ae-b0bc713a9d9d",
        "robot_profile": {
            "name": "S2_L04"
        }
    },
    {
        "robot_id": "b011ee0f-9989-4c67-84c1-603550b9197e",
        "robot_profile": {
            "name": "S2_L05"
        }
    },
    {
        "robot_id": "c1215bea-90b1-4912-8c72-21a811e3276e",
        "robot_profile": {
            "name": "S2_L06"
        }
    },
    {
        "robot_id": "9f871da5-7d46-4da5-b6aa-92f4481ae6d4",
        "robot_profile": {
            "name": "S2_L07"
        }
    },
    {
        "robot_id": "47400ccb-56db-4072-b0c9-3fa41de0d474",
        "robot_profile": {
            "name": "S2_L08"
        }
    },
    {
        "robot_id": "457dd728-2d39-4eca-b938-67ff44b94d69",
        "robot_profile": {
            "name": "S2_L09"
        }
    },
    {
        "robot_id": "dc41c38b-6f3a-4c7f-a319-684979446c70",
        "robot_profile": {
            "name": "S2_L10"
        }
    },
    {
        "robot_id": "be963884-4a82-4416-9aa6-96c6c1f740c7",
        "robot_profile": {
            "name": "S2_L11"
        }
    },
    {
        "robot_id": "efdf6d02-6fc2-4be3-96cd-f27d8a895d23",
        "robot_profile": {
            "name": "S2_L12"
        }
    },
    {
        "robot_id": "ffdab87a-57d8-4738-8c87-d7470339073b",
        "robot_profile": {
            "name": "S2_L13"
        }
    },
    {
        "robot_id": "c3459cd5-e041-4310-98aa-5dbb7f2590e6",
        "robot_profile": {
            "name": "S2_L14"
        }
    },
    {
        "robot_id": "da0ed3fd-4d1b-4d12-a8a5-821d19087bd2",
        "robot_profile": {
            "name": "S2_L15"
        }
    },
    {
        "robot_id": "55bff982-b916-4c5d-9599-088e247cec19",
        "robot_profile": {
            "name": "S2_L16"
        }
    },
    {
        "robot_id": "f05d4cef-e25f-4926-9bb3-4eafc2e33e78",
        "robot_profile": {
            "name": "S2_L17"
        }
    },
    {
        "robot_id": "49d7b1a1-7009-4fb0-9170-7187afec00bf",
        "robot_profile": {
            "name": "S2_L18"
        }
    },
    {
        "robot_id": "4896f4f7-eccf-40df-8f66-cff4ee21ad1e",
        "robot_profile": {
            "name": "S2_L19"
        }
    },
    {
        "robot_id": "76597d29-4f47-44d7-a466-388e9a6a0833",
        "robot_profile": {
            "name": "S2_L20"
        }
    },
    {
        "robot_id": "e4432837-8a37-4e25-8557-e1fcbfdb41c6",
        "robot_profile": {
            "name": "S2_L21"
        }
    },
    {
        "robot_id": "1705899a-05f4-4d12-84bf-5a91380b4bbb",
        "robot_profile": {
            "name": "S2_L22"
        }
    },
    {
        "robot_id": "ade31e24-9ed4-4ae0-b4ea-196e33e05a49",
        "robot_profile": {
            "name": "S2_L23"
        }
    },
    {
        "robot_id": "39613fc1-3ac7-4d5c-91f7-3b7e047e79be",
        "robot_profile": {
            "name": "S2_L24"
        }
    },
    {
        "robot_id": "29c52e6f-be98-4846-be32-176f8471a4d8",
        "robot_profile": {
            "name": "S2_L25"
        }
    }
];

HB1 = [
    {
        "robot_id": "82d274e9-a690-4b48-9174-233b2526039a",
        "robot_profile": {
            "name": "B1_L01"
        }
    },
    {
        "robot_id": "ce733ffa-5404-420c-aecc-5fccfa49ade3",
        "robot_profile": {
            "name": "B1_L02"
        }
    },
    {
        "robot_id": "25f952a9-8741-4636-a7d3-1de943a27dd1",
        "robot_profile": {
            "name": "B1_L03"
        }
    },
    {
        "robot_id": "1099ea61-18c1-4675-90d1-f42933d762f9",
        "robot_profile": {
            "name": "B1_L04"
        }
    },
    {
        "robot_id": "2133f8b4-5bb1-40f2-a762-8ae716d3c026",
        "robot_profile": {
            "name": "B1_L05"
        }
    },
    {
        "robot_id": "c581106b-dbed-4d6e-96b6-f65214350404",
        "robot_profile": {
            "name": "B1_L06"
        }
    },
    {
        "robot_id": "054374b3-0705-459c-b092-1bb9934fe7a4",
        "robot_profile": {
            "name": "B1_L07"
        }
    },
    {
        "robot_id": "f2888c2e-0470-41bc-a646-5eafe517899a",
        "robot_profile": {
            "name": "B1_L08"
        }
    },
    {
        "robot_id": "94418edf-1afc-4779-8fb3-eb30353c0387",
        "robot_profile": {
            "name": "B1_L09"
        }
    },
    {
        "robot_id": "6f98d412-30ad-40ff-ab04-b000a0d716aa",
        "robot_profile": {
            "name": "B1_L10"
        }
    },
    {
        "robot_id": "2fba3160-397c-4835-a3f4-2cd54ab9c3f5",
        "robot_profile": {
            "name": "B1_L11"
        }
    },
    {
        "robot_id": "2d8f8267-0d52-4b7a-9ee1-1bdc1575d9f7",
        "robot_profile": {
            "name": "B1_L12"
        }
    },
    {
        "robot_id": "857a12c2-4ab8-4ab7-897b-ba0aad0ec7a1",
        "robot_profile": {
            "name": "B1_L13"
        }
    },
    {
        "robot_id": "2e11f44e-5719-43db-bf55-8363895ea312",
        "robot_profile": {
            "name": "B1_L14"
        }
    },
    {
        "robot_id": "b792ba40-1411-4302-b40d-21bc84cfb860",
        "robot_profile": {
            "name": "B1_L15"
        }
    },
    {
        "robot_id": "7b7f3416-1734-4bb2-850e-2217b64b0bbf",
        "robot_profile": {
            "name": "B1_L16"
        }
    },
    {
        "robot_id": "664a3219-6c3e-43b9-87d8-07d247fda1a5",
        "robot_profile": {
            "name": "B1_L17"
        }
    },
    {
        "robot_id": "16e08105-e4ae-4418-983f-e100522203ef",
        "robot_profile": {
            "name": "B1_L18"
        }
    },
    {
        "robot_id": "b152e07c-af3b-45e6-9dc2-d1d2dd7734cf",
        "robot_profile": {
            "name": "B1_L19"
        }
    },
    {
        "robot_id": "71c87d61-9a29-41e6-afb3-0504a3cc96b6",
        "robot_profile": {
            "name": "B1_L20"
        }
    },
    {
        "robot_id": "23fd70ae-b10f-4a3b-8d9e-99209906dcf2",
        "robot_profile": {
            "name": "B1_L21"
        }
    },
    {
        "robot_id": "58689eb2-d8cb-4e7e-9744-fbaa990013ea",
        "robot_profile": {
            "name": "B1_L22"
        }
    },
    {
        "robot_id": "1a113ff7-9cf8-41be-8a75-09e506d37cfd",
        "robot_profile": {
            "name": "B1_L23"
        }
    },
    {
        "robot_id": "aa39f7b8-9964-4cb2-a9f4-c82f1b17c3c6",
        "robot_profile": {
            "name": "B1_L24"
        }
    },
    {
        "robot_id": "b5f86734-6986-4638-881a-6e9c37ae55fd",
        "robot_profile": {
            "name": "B1_L25"
        }
    }
];

HB2 = [
    {
        "robot_id": "d6bc0869-5ebc-4c2f-9638-710a29c4e651",
        "robot_profile": {
            "name": "B2_L01"
        }
    },
    {
        "robot_id": "6a7f968e-90b8-43e2-a59c-e771db6311d1",
        "robot_profile": {
            "name": "B2_L02"
        }
    },
    {
        "robot_id": "05c51d1d-da71-40b6-a577-a59076c63f33",
        "robot_profile": {
            "name": "B2_L03"
        }
    },
    {
        "robot_id": "9cd62312-3f23-40d8-92dc-d77b62f49d12",
        "robot_profile": {
            "name": "B2_L04"
        }
    },
    {
        "robot_id": "c59f6e11-67e5-4445-a504-51130cbfb953",
        "robot_profile": {
            "name": "B2_L05"
        }
    },
    {
        "robot_id": "2839a2be-d1fd-4ecf-8a76-435cb19ba1ea",
        "robot_profile": {
            "name": "B2_L06"
        }
    },
    {
        "robot_id": "f4da7e93-54e4-4733-8576-0351b775d604",
        "robot_profile": {
            "name": "B2_L07"
        }
    },
    {
        "robot_id": "f34f7be8-4f43-43b8-8085-681f5bc3ca9e",
        "robot_profile": {
            "name": "B2_L08"
        }
    },
    {
        "robot_id": "81e7f282-8741-48c7-b7bf-c1076aa4b179",
        "robot_profile": {
            "name": "B2_L09"
        }
    },
    {
        "robot_id": "4a67b0cb-ca64-44ac-a6c2-998fc8309050",
        "robot_profile": {
            "name": "B2_L10"
        }
    },
    {
        "robot_id": "831e47e2-bc76-442c-92a1-ca48f0c165fc",
        "robot_profile": {
            "name": "B2_L11"
        }
    },
    {
        "robot_id": "001c9d09-e425-4f0d-b297-3a60b7e299a2",
        "robot_profile": {
            "name": "B2_L12"
        }
    },
    {
        "robot_id": "1225c8cc-a303-468b-8d18-ac8123b5c9be",
        "robot_profile": {
            "name": "B2_L13"
        }
    },
    {
        "robot_id": "71e7949f-a629-45cd-ae6b-1e6874035bf5",
        "robot_profile": {
            "name": "B2_L14"
        }
    },
    {
        "robot_id": "376561b4-3209-454f-9954-67ec1e489d23",
        "robot_profile": {
            "name": "B2_L15"
        }
    },
    {
        "robot_id": "9575122c-fe8e-44c1-a127-3feeccb28800",
        "robot_profile": {
            "name": "B2_L16"
        }
    },
    {
        "robot_id": "18f34c65-f216-48d2-82ec-ed2a3008c7f9",
        "robot_profile": {
            "name": "B2_L17"
        }
    },
    {
        "robot_id": "129cd076-b890-44bb-9084-9cbd73674d9f",
        "robot_profile": {
            "name": "B2_L18"
        }
    },
    {
        "robot_id": "05c0acfc-fa9f-4c2f-8082-eb52db71a2d1",
        "robot_profile": {
            "name": "B2_L19"
        }
    },
    {
        "robot_id": "2b37e1ec-f12b-4c97-a170-5b998b9a50bf",
        "robot_profile": {
            "name": "B2_L20"
        }
    },
    {
        "robot_id": "54bcbb3b-bd1e-4b73-a70d-80b541a0013f",
        "robot_profile": {
            "name": "B2_L21"
        }
    },
    {
        "robot_id": "917bc00f-87a8-44e5-b0fd-2a47d4fee999",
        "robot_profile": {
            "name": "B2_L22"
        }
    },
    {
        "robot_id": "9e7bcd35-7bab-4963-a830-69bc82ecaefa",
        "robot_profile": {
            "name": "B2_L23"
        }
    },
    {
        "robot_id": "cc4d7f33-2b62-4de9-b619-ed8db6b5f71f",
        "robot_profile": {
            "name": "B2_L24"
        }
    },
    {
        "robot_id": "7cde8d4d-d3be-4ffc-9c40-46bc4d3a185a",
        "robot_profile": {
            "name": "B2_L25"
        }
    }
];

// let group_id = "ce5de171-e7dc-424e-a34e-c4873ab76cb3";
// GROUP_ID = "3b4c4e2d-1487-48ca-9e34-76ef99b9ce85"; // PRIMARIA 1B 28-01-2025

// GROUP_ID = "f09f151c-ec01-4c74-94dc-ec570377b5d6";    //Secundaria 1B 30-01-2025
// GROUP_ID = "d747680e-4aa2-403b-9bb7-35018de5f9fd";    //Secundaria 2B 30-01-2025
// GROUP_ID = "0816afbf-3d86-47b0-aa96-cc5cda4912c5";    //Bachillerato 1B 30-01-2025
GROUP_ID = "d4d9047c-d2cf-40e6-97f9-71d873a7a337";    //Bachillerato 2B 30-01-2025

HB2.forEach(robot => {
    let cmd = `npm run aker-gql-op -- robot_group_add  --robot_id ${robot.robot_id} --group_id ${GROUP_ID} --robot_type B2`;
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
});