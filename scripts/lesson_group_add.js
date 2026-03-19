const {
  exec
} = require("child_process");
const fs = require('fs');
K4 = [
  ['f7a6c4a3-3b42-47bb-b6f3-9a4552fa4aba', 'K4-L01-Tengo un nuevo kit para jugar y aprender.pdf'],
  ['b22d57ad-29c6-4a61-bf23-694953d77e3a', 'K4-L02-¡Vamos al parque!.pdf'],
  ['9ca8b0f8-acb7-4d00-81e3-5e2935c194d0', 'K4-L03-Cuéntame un cuento.pdf'],
  ['2aac2612-12b8-418c-a204-5a3b41df95fa', 'K4-L04-Juguemos a ser exploradores.pdf'],
  ['38235859-74d2-4b5b-bee9-86115569db3c', 'K4-L05-Nazco, crezco y.pdf'],
  ['b97f1251-7c8c-468b-8fe7-f49f77b22283', 'K4-L06-Rojo significa alto.pdf'],
  ['a7997617-fd6a-46c4-8c02-b1b896b3ea26', 'K4-L07-Son carros que vuelan.pdf'],
  ['d5ef2d1b-3fad-43bb-9c8a-9b55e7e8fcec', 'K4-L08-Gira y gira la rueda.pdf'],
  ['bd62eae4-84d8-4bd8-acf9-ce7b3b11ec5a', 'K4-L09-Con agua y con jabón.pdf'],
  ['c5770adb-f0d2-42c7-8694-8fcdc40e05ac', 'K4-L10-Abejitas al panal.pdf'],
  ['7586a86f-5ede-4df8-93e3-1f4afddd4dc3', 'K4-L11-Vamos al súper.pdf'],
  ['74d4b95a-3bce-41fa-bb75-d452ea29bfee', 'K4-L12-Manitas creativas.pdf'],
  ['c217dbfe-a0f5-437b-9f4a-3b87e2bd8db1', 'K4-L13-Cantemos el himno nacional.pdf'],
  ['0c14eb8d-2efc-484c-95b6-00c0177c7a30', 'K4-L14-Exploremos otro país.pdf'],
  ['6a6a2903-eb3f-4937-944d-d46c274b052f', 'K4-L15-No todos tienen patitas.pdf'],
  ['150493c1-1718-4b33-91ba-bd9f59c4305e', 'K4-L16-Conozcamos más animalitos.pdf'],
  ['ba2e94f9-f485-4102-b958-44f9bc3ed3a6', 'K4-L17-Una excursión por mi colegio.pdf'],
  ['3a2e6513-5289-4c48-8e1d-6c60226f913c', 'K4-L18-Cuéntame lo que pasó.pdf'],
  ['8cafc0d8-08dd-46d9-9c1a-dfdef133ed56', 'K4-L19-Juguemos a ser adultos.pdf'],
  ['752d5c4d-65ef-498f-8ae9-db0a7c59b093', 'K4-L20-Dime cuál es esta figura.pdf'],
  ['a8e3b667-8e8a-46f3-8108-785265635ff9', 'K4-L21-Trencito chucu, chucu, chú....pdf'],
  ['7a719d83-67e3-473a-8c79-fe6275bb88b3', 'K4-L22-Ese extraño no.pdf']
]

K5 = [
  ['835bf77d-7ca4-4910-ab9a-2decc8f80b48', 'K5-L01-¡Este es mi kit!.pdf'],
  ['6837e519-dbfa-46d9-bd32-9921bef369ff', 'K5-L02-Juguemos a ser constructores.pdf'],
  ['087ad365-9ba6-4c0f-b2e8-d723432b0f7e', 'K5-L03-Visitemos la ferretería.pdf'],
  ['1652859c-ee08-4f2b-8875-669415008a6e', 'K5-L04-Así se ocupan las tijeras.pdf'],
  ['25b96620-b251-48de-a31c-70bc70ab7d6c', 'K5-L05-Así viven las hormigas.pdf'],
  ['2fd34e85-e552-48ba-823a-c39dac215bb9', 'K5-L06-Vamos al parque.pdf'],
  ['d18d337d-2f2b-46a0-8610-13376fc0930f', 'K5-L07-Cuéntame lo sucedido.pdf'],
  ['116a9f6c-8ec5-4c6a-8c10-c2bc8b7621bd', 'K5-L08-Qué hay en mi comunidad.pdf'],
  ['ca304019-87f3-494c-8e9d-8f276ba95b20', 'K5-L09-Imitemos los sonidos.pdf'],
  ['c30ab733-3460-4604-be75-cb33c3e0b104', 'K5-L10-Mi casa y lo que hay en ella.pdf'],
  ['ee7ee7fd-6948-4da1-bdbe-367de3ba7a04', 'K5-L11-Había una vez.pdf'],
  ['7bd4d9ef-9708-4a90-baad-0a29d4e17ddb', 'K5-L12-Juguemos a ser constructores.pdf'],
  ['efafda26-6dd6-4d2f-8d82-081b144f964c', 'K5-L13-Cuido de las plantas.pdf'],
  ['b8214a91-29b3-4252-b599-159641d5866d', 'K5-L14-Hace mucho frío.pdf'],
  ['f23a5254-ad41-497d-b89e-697a6cdb8625', 'K5-L15-Este es mi hogar.pdf'],
  ['cbc6d41a-ee39-49c6-bcb4-5ca056ebf539', 'K5-L16-Pido ayuda si estoy en problemas.pdf'],
  ['555007e1-af0c-456f-936f-af56afe57782', 'K5-L17-La familia abeja.pdf'],
  ['d8b6e8d9-e2ef-40d2-8232-b0ee3a72c4c3', 'K5-L18-Bajo el mar....pdf'],
  ['7d3ac985-36fa-4d2d-87e0-325ae79a8866', 'K5-L19-Esto no me gusta.pdf'],
  ['85aa5c78-6324-4ece-86eb-39fe8c31bab3', 'K5-L20-Este insecto no me gusta.pdf'],
  ['6eb42734-b505-4350-9401-e88b7ab4307e', 'K5-L21-Conozcan mi mascota.pdf'],
  ['c7e09c66-2e88-49a0-9aad-9dab4e9b8c8f', 'K5-L22-Se esconde y come queso.pdf']
]

PP = [
  ['869169c8-da0d-497a-8366-c0c34dcbb663', 'PP-L01-A qué se parece.pdf'],
  ['da107366-b1c8-441c-944c-c2430a72dee0', 'PP-L02-Conozcamos sobre el cuerpo humano.pdf'],
  ['30f1916d-1a8f-486b-8f7f-14e9ec9c20e9', 'PP-L03-Posicionemos objetos.pdf'],
  ['64423934-3ce6-4c04-ba84-965f21952a95', 'PP-L04-Aprendamos a ocupar la cinta métrica.pdf'],
  ['c810c776-8c37-4b28-b3d4-b001493ef266', 'PP-L05-Ganamos la carrera.pdf'],
  ['be2d346e-19ad-4455-8524-3563fa153855', 'PP-L06-Cuenta sus lados y adivina qué es.pdf'],
  ['df634e6e-ab3e-4f88-8af9-b055b2f622c6', 'PP-L07-Visitemos el centro comercial.pdf'],
  ['84a61120-f18b-4188-abba-c3fc9fb6f597', 'PP-L08-Seamos constructores por un día.pdf'],
  ['4ed23fdd-0834-4140-89a8-98421295ae5f', 'PP-L09-Préstame tu caña de pescar.pdf'],
  ['dd244812-b997-4a0d-81d6-c362fa566453', 'PP-L10-Vamos a la feria.pdf'],
  ['c3611deb-ad55-4ef0-9dec-016d8ebbb8c0', 'PP-L11-Incendio en el bosque!.pdf'],
  ['559d5402-265e-4058-8dde-f92c2f7f78f4', 'PP-L12-Volemos como las aves.pdf'],
  ['effe40ad-8f94-44ee-ba13-55a7d94fd11f', 'PP-L13-Exploradores del desierto.pdf'],
  ['7c55dbf5-2e52-4983-9bf7-a2be0ad3a7ae', 'PP-L14-La semilla mágica.pdf'],
  ['256efedd-20ac-4930-be63-3d99110d6e0d', 'PP-L15-Nacemos, crecemos, ¿y luego.pdf'],
  ['a9eeedee-16ff-41ce-af95-691b17cf30aa', 'PP-L16-Aprendo la canción.pdf'],
  ['365f914e-7524-4bfb-ab94-06ac5b97dad1', 'PP-L17-Cuidado ¡Fuego!.pdf'],
  ['834b187e-4792-4456-8084-5540546e8d2a', 'PP-L18-Todos somos investigadores.pdf'],
  ['742dceb3-18bb-44fe-8c0e-c886fcd2d9e4', 'PP-L19-Experimentos divertidos.pdf'],
  ['9af3abbc-e6b8-4a64-950a-0d7c3b6174bf', 'PP-L20-Vamos a la mar.pdf'],
  ['dcdf5dab-7760-41f1-bba1-5f1e81fe5d03', 'PP-L21-Tiremos la basura en su lugar.pdf'],
  ['6525e4cf-ad44-4026-921c-0ef2f6dba5b7', 'PP-L22-Cuidado con el viento.pdf']
]

P1 = [
  ['660db88f-91b7-422b-8ad2-8b79e93975ba', 'P1-L01-¡Conozcamos nuestras piezas!.pdf'],
  ['e616057f-0ed7-44af-bc18-47a9139c29db', 'P1-L02-Aprendamos a orientar y posicionar objetos.pdf'],
  ['4a704e40-fc7a-4da9-ba41-1d4e6ad3ac84', 'P1-L03-Construyamos castillos.pdf'],
  ['74b72416-511b-4593-91fe-fe500c6f64c8', 'P1-L04-Conozcamos las máquinas simples.pdf'],
  ['2f673fcd-bb95-400a-8df0-9f84e5377f23', 'P1-L05-Conozcamos sobre el plano inclinado.pdf'],
  ['2d8a71ef-3fe5-43fc-9060-c4e0b05f8ac3', 'P1-L06-Clasifiquemos y tracemos líneas.pdf'],
  ['81f6c4e7-d432-4d02-8256-1561ba5dd810', 'P1-L07-Armemos pistas de carrera.pdf'],
  ['8dc0282a-8965-46a1-9b88-91cacf68da48', 'P1-L08-Aprendamos sobre símbolos patrios.pdf'],
  ['0150ffb0-48e9-4f66-9c00-27788476ba99', 'P1-L09-Conozcamos sobre poleas.pdf'],
  ['c5c0dead-e548-406c-8a0a-15edad8df716', 'P1-L10-Juguemos a ser pescadores.pdf'],
  ['67e8586a-8a51-4649-ab10-fcccc2c8cec7', 'P1-L11-Así funcionan las ruedas.pdf'],
  ['fde8dc8f-c98c-4b6f-b67b-bd45adb03741', 'P1-L12-Manejemos helicópteros.pdf'],
  ['0074cd94-ccd3-4ce3-97eb-62b7b1095b55', 'P1-L13-Adivina, adivinador.pdf'],
  ['6931ce79-b566-42f9-abad-2f7339193656', 'P1-L14-Busquemos semejanzas y diferencias animales.pdf'],
  ['7a04e34a-f66d-4e02-93a1-2a1c2a79a551', 'P1-L15-Ideemos y clasifiquemos estructuras.pdf'],
  ['b8678d3b-4b54-4265-a0fb-d14959697580', 'P1-L16-Cómo funcionan los molinos de viento.pdf'],
  ['ed6f208e-d1b9-4de4-8fec-4b226f9c8548', 'P1-L17-Encajemos los engranajes.pdf'],
  ['26ff6991-3f7b-44d6-ae78-489f4dcdd178', 'P1-L18-Giremos la manivela.pdf'],
  ['f9b4a321-4389-48dc-be6a-b98cb381fdda', 'P1-L19-Describamos nuestro hogar.pdf'],
  ['7408e4af-15db-4b67-9c5e-1f47fef8bb16', 'P1-L20-Compitamos en una carrera de carros.pdf'],
  ['53a21341-7874-4775-8a41-145b9ed4fab8', 'P1-L21-El trencito, chucu, chucu, chu….pdf'],
  ['956bbb59-7100-484e-9a22-7a3f9b9ee695', 'P1-L22-Cuánta fuerza tiene el agua.pdf']
]

P2 = [
  ['479dc116-91c9-49c2-843a-fe733f0f5893', 'P2-L01-Con qué piezas puedo jugar.pdf'],
  ['368b8bb9-e8d8-42a1-9162-f47cd4af3884', 'P2-L02-¡Movámonos!.pdf'],
  ['6b8526a6-5b7c-4c9f-bd11-2e6de3df0343', 'P2-L03-Qué es una mantis.pdf'],
  ['0846d21c-ca28-4880-b8f3-6c4eb32f5e81', 'P2-L04-Qué son las palancas.pdf'],
  ['01a9f1e2-2180-4258-9aec-f1afa76d9dd5', 'P2-L05-Juguemos con el tobogán.pdf'],
  ['e33e3360-cf73-486f-8c98-a8147eab2d51', 'P2-L06-Aprendamos sobre gravedad.pdf'],
  ['fb4696ab-764c-496e-912e-7af641314104', 'P2-L07-Pongámosle reglas al juego.pdf'],
  ['7bcfa181-c41b-490f-ae38-52913cb3a199', 'P2-L08-Construyamos poleas.pdf'],
  ['db11e294-a797-47d9-928a-05f0edd8f26b', 'P2-L09-Qué son los contrapesos.pdf'],
  ['439379a9-531f-4cbd-9d0c-76623a2ab11f', 'P2-L10-Así funcionan las grúas.pdf'],
  ['30fb2e42-8919-4e8c-b072-381300fe87b8', 'P2-L11-Avioncito, vuela.pdf'],
  ['38607d14-bb9b-46e0-a375-d9a601d29bad', 'P2-L12-Armemos un montacargas.pdf'],
  ['4c802698-b035-4cd2-83fe-6528a1cd9afc', 'P2-L13-Imaginemos el final del cuento.pdf'],
  ['f1aaafdf-69b9-405b-90d1-b4bcf022caa8', 'P2-L14-Construyo medios de transporte terrestre.pdf'],
  ['b51f2bb8-4481-4dd0-95d4-57a8a7ad4016', 'P2-L15-Castillos y fortalezas.pdf'],
  ['45c12d70-1e00-4728-9188-3914e14f61f3', 'P2-L16-Y la moraleja de esta historia es....pdf'],
  ['db4eb1d9-83c8-453f-941d-c214a913c43c', 'P2-L17-Qué son los engranajes.pdf'],
  ['d279e7cf-9de5-40c6-9895-82879d5dfc9f', 'P2-L18-Calculemos el ángulo.pdf'],
  ['5420632c-feaf-4cac-978f-ef04c72a6bb8', 'P2-L19-Conozcamos sobre industria y producción.pdf'],
  ['a438a563-1828-4874-8647-28b8e5f2ec8d', 'P2-L20-Vamos a la mar....pdf'],
  ['a77f6dca-d647-4d15-a340-53ea9fe85796', 'P2-L21-¿Se descargaron tus baterías.pdf'],
  ['c21c803a-646f-45cb-8751-e14012f64644', 'P2-L22-Hagamos una competencia.pdf']
]

P3 = [
  ['d0ddf4b6-0bdd-45cb-af90-322cd4b8804e', 'P3-L01-¡Conozcamos nuestro kit!.pdf'],
  ['ccb8f3ef-9878-4c17-b580-a3efd47197b7', 'P3-L02-¡Busquemos ángulos!.pdf'],
  ['6f96bedb-92ed-4e40-8fb3-9a086306ba97', 'P3-L03-Aprendamos a ocupar la balanza.pdf'],
  ['e2d0e7f6-bbec-4789-b554-be4532949387', 'P3-L04-Construyamos palancas.pdf'],
  ['5c87a349-7c6f-428e-84ab-6679110edda0', 'P3-L05-Describo mi vivienda familiar.pdf'],
  ['0e5bbc7b-c268-4c18-b2e9-335039d4d7f8', 'P3-L06-Armemos un plano inclinado.pdf'],
  ['1089e3c2-803f-49ef-ab4f-115a3f5b4990', 'P3-L07-Dejemos que nos enseñe Newton.pdf'],
  ['c6921137-fb72-42e3-aff7-6dd09c87fcb0', 'P3-L08-Aprendo a armar poleas.pdf'],
  ['d1e179b2-c438-40fc-b936-b698429afbeb', 'P3-L09-Aprendamos a ser constructores.pdf'],
  ['c40a3412-6f1b-4ca2-ad19-3132d2b2b3f0', 'P3-L10-Conozco leyendas de mi país.pdf'],
  ['190ed057-8fad-4a35-94a0-cde2f0557384', 'P3-L11-Vamos a la feria.pdf'],
  ['16144f9f-f30a-4349-862f-9e6712391dca', 'P3-L12-Colaboro en el aseo y el arreglo de mi hogar - copia.pdf'],
  ['295969af-bd42-4c85-99c1-886dd17fd6fc', 'P3-L13-Mecánico por un día.pdf'],
  ['dddcad22-7234-49ee-8b68-7fd5261d3a95', 'P3-L14-Todos los vehículos son importantes.pdf'],
  ['a1b72243-f6a9-484d-baa2-c566f422d677', 'P3-L15-Cuánta energía produce el viento.pdf'],
  ['f6f4ad58-5ce0-4757-8823-bb16b3960421', 'P3-L16-Cuántas estructuras puedo armar.pdf'],
  ['b0b16bec-7361-427f-bc8c-a299bf2b53c8', 'P3-L17-Juguemos con engranajes.pdf'],
  ['ebea8941-7ce1-4c18-8ee8-887ab787eb93', 'P3-L18-Saquemos nuestra bicicleta.pdf'],
  ['07fc7f38-56cd-4021-bd8e-cc31af69083e', 'P3-L19-Vamos de compras a la ferretería.pdf'],
  ['189b997a-0777-4e8f-b19e-085b00f1961a', 'P3-L20-Robótica por todos lados.pdf'],
  ['1bb9315e-9977-4bb6-8700-94bf5b6248ac', 'P3-L21-¿Y si le ponemos un motor.pdf'],
  ['9bd1b16f-4666-470f-aae0-2643305678e7', 'P3-L22-De dónde viene la energía eléctrica.pdf']
]

P4 = [
  ['dddcad22-7234-49ee-8b68-7fd5261d3a95', 'P3-L14-Todos los vehículos son importantes.pdf'],
  ['a1b72243-f6a9-484d-baa2-c566f422d677', 'P3-L15-Cuánta energía produce el viento.pdf'],
  ['f6f4ad58-5ce0-4757-8823-bb16b3960421', 'P3-L16-Cuántas estructuras puedo armar.pdf'],
  ['b0b16bec-7361-427f-bc8c-a299bf2b53c8', 'P3-L17-Juguemos con engranajes.pdf'],
  ['ebea8941-7ce1-4c18-8ee8-887ab787eb93', 'P3-L18-Saquemos nuestra bicicleta.pdf'],
  ['07fc7f38-56cd-4021-bd8e-cc31af69083e', 'P3-L19-Vamos de compras a la ferretería.pdf'],
  ['189b997a-0777-4e8f-b19e-085b00f1961a', 'P3-L20-Robótica por todos lados.pdf'],
  ['c3ffda5c-6e9f-41de-9f1d-bae94b7a9dad', 'P4-L01-¡Este es nuestro kit!.pdf'],
  ['57c68bb1-6ac8-4ef4-9149-ad22fe325230', 'P4-L02-Aprendamos de gravedad.pdf'],
  ['cdb824e8-121c-408b-ad6b-dfa85f99a298', 'P4-L03-Construyamos máquinas medievales.pdf'],
  ['adf5ffb1-72c9-4979-96db-3b646e5894a0', 'P4-L04-Así funcionan mis manos.pdf'],
  ['159c8fc4-92d9-47e0-86d7-8edc2960d29a', 'P4-L05-Conozcamos sobre planos inclinados.pdf'],
  ['caf28460-f006-4972-8265-7135d714f742', 'P4-L06-Es recta o perpendicular.pdf'],
  ['7b4608be-1053-42b3-a719-d3b97341bb34', 'P4-L07-Fuerza a distancia.pdf'],
  ['9c15a0cd-828c-40f5-af57-130fb35bb263', 'P4-L08-Cómo funcionan las poleas.pdf'],
  ['b310f56c-ca64-430f-b509-be2ba0cbc09e', 'P4-L09-Juguemos a ser constructores.pdf'],
  ['857b1080-97c0-4300-b28b-885ae398c896', 'P4-L10-Subes o bajas.pdf'],
  ['29f2bab3-79dd-4633-bd90-97d9f972fec4', 'P4-L11-Armemos nuestro propio drone.pdf'],
  ['ca74e01c-40b0-4af0-9325-fb5ae0fc9c04', 'P4-L12-Motocross.pdf'],
  ['1326c682-9825-494d-aca3-d875fa5ef2c0', 'P4-L13-De dónde proviene la energía eléctrica.pdf'],
  ['1d2c98af-a3ce-43f4-a198-55e0041db0f3', 'P4-L14-Identifiquemos y midamos ángulos.pdf'],
  ['79e02997-d651-4a56-8e79-02a8c9604a9d', 'P4-L15-Clasifiquemos estructuras.pdf'],
  ['c597b7a3-e6ba-4be1-805c-1d2a90f92b41', 'P4-L16-Conozcamos los medios de transporte terrestre.pdf'],
  ['2d4c93f0-2326-4d08-b67f-76e27abcf8f5', 'P4-L17-Armemos una caja de velocidades.pdf'],
  ['3bab66f4-c654-4778-b3ab-2a93464e53e4', 'P4-L18-Conozcamos sobre engranajes.pdf'],
  ['06eaa084-ad30-47b0-809e-53c75026c168', 'P4-L19-Redacta un manual.pdf'],
  ['279239cf-3800-45f2-bfe2-21312889d13b', 'P4-L20-Está temblando, pero qué hago.pdf'],
  ['6bb42aad-f139-402c-82ff-7087fd087395', 'P4-L21-¡Ejercitémonos.pdf'],
  ['63b11e01-8ce0-4a89-b229-a1867108fd63', 'P4-L22-Viajemos en tren.pdf']
]

P5 = [
  ['9423ca6b-246b-40d9-8ecf-7d1bfca68794', 'P5-L01-¡Ayudemos al castor!.pdf'],
  ['b0ea07a5-45de-487a-8e05-9cf26c9e5d4e', 'P5-L02-Sigamos manuales.pdf'],
  ['f653bb04-8952-40d7-9fee-ea0eb803914e', 'P5-L03-Ideemos textos descriptivos.pdf'],
  ['868703ee-d438-4e7f-87f6-9e28a4eea43c', 'P5-L04-Busquemos palancas dentro de las maquinarias.pdf'],
  ['6efbec91-8648-4125-881f-dbe5d019e5b5', 'P5-L05-Hagamos un manual.pdf'],
  ['1b8ba719-e80f-4b99-a8be-5f401e458c5f', 'P5-L06-Es o no un círculo.pdf'],
  ['7885ca1f-4577-49d9-b3c8-f94008335220', 'P5-L07-Aprendamos a medir ángulos.pdf'],
  ['206f4078-7bd3-422a-be2a-a0eff2b84ca9', 'P5-L08-Iniciemos la costrucción.pdf'],
  ['9646e9e3-f09e-42b1-a715-8e94e6f075a2', 'P5-L09-Peso para bajar y contrapeso para subir.pdf'],
  ['8172aa90-7fe7-4ee5-95bc-8ec1dd6a6883', 'P5-L10-Construyamos poleas.pdf'],
  ['0d5cbecf-c1e7-440f-903c-df720e9a6fa3', 'P5-L11-Construyamos con ruedas.pdf'],
  ['9e0ae4bf-769d-4aae-8522-6e6898093cbb', 'P5-L12-Organicemos una competencia.pdf'],
  ['da8b6499-afcd-4b1d-87c6-4f72f31feeaf', 'P5-L13-Bajo el mar.pdf'],
  ['1dd6606f-1f3c-4276-86bf-4a1c3244deb9', 'P5-L14-Construyo medios de transporte terrestre.pdf'],
  ['30c43abe-10a0-41da-bf32-0eafb541457a', 'P5-L15-Castillos y fortalezas.pdf'],
  ['80492890-0e64-4911-8dae-b6dae5bed45d', 'P5-L16-Y la moraleja de esta historia es....pdf'],
  ['da77fb15-e7c0-467f-8489-5fbf5219f5ea', 'P5-L17-Aprendo sobre engranajes.pdf'],
  ['eadf3179-4b05-4595-afb7-be3d95ebcca1', 'P5-L18-Qué hacer cuando está temblando.pdf'],
  ['0a130d05-411b-40a3-bb4a-02b238421894', 'P5-L19-Ensamblo máquinas compuestas.pdf'],
  ['73864a51-54c0-4549-aefe-a7f38e1b989d', 'P5-L20-Cuánto conozco sobre energía.pdf'],
  ['1b1a188b-70de-4333-9942-c6d2374d296b', 'P5-L21-Costumbres griegas.pdf'],
  ['a47b694b-9e79-428b-905e-9de74761d731', 'P5-L22-Describamos una época.pdf']
]

P6 = [
  ['3df7166e-69f6-4c46-aad3-142d27fcc500', 'P6-L01-Aprendamos sobre redes viales.pdf'],
  ['ba52bfb8-d890-46a1-9163-380a3563f140', 'P6-L02-Ejes de simetría.pdf'],
  ['cfcf4e44-847b-4ab0-ba52-7ff076f48c33', 'P6-L03-Cómo funcionan las palancas en mi armado.pdf'],
  ['fba91b68-14a9-4b68-a136-6c6001228a53', 'P6-L04-Organicemos una competencia.pdf'],
  ['e2341ea6-f001-494a-b9b8-6b860f81fee8', 'P6-L05-Armemos planos inclinados.pdf'],
  ['d4a21386-f4d8-42e0-928b-d23042265e33', 'P6-L06-Acaso este es un tornillo.pdf'],
  ['02acff60-14e9-42da-ba0c-0731785100b2', 'P6-L07-Aquí es donde deposito la basura.pdf'],
  ['eb290c9e-cf04-4e1b-816d-2429c3fe3202', 'P6-L08-Quién será el ganador.pdf'],
  ['4c8a87b4-d9ac-48d6-ac0e-06ca499f4b38', 'P6-L09-Construyamos con poleas.pdf'],
  ['8a1baa9a-f228-409f-add9-e1517a27e3d7', 'P6-L10-Teleférico o funicular.pdf'],
  ['49faf23c-30b3-4587-bee2-26e6887a8f21', 'P6-L11-La robótica del ahora y del mañana.pdf'],
  ['ee0b9cc4-08a4-4086-9411-978384d762dc', 'P6-L12-Conozcamos figuras geométricas.pdf'],
  ['76d4c4f5-301d-4a64-bc75-da592ab251f3', 'P6-L13-Cómo se almacena la energía.pdf'],
  ['05c75225-accb-45ff-b7c1-74e75ad5b38e', 'P6-L14-Todos los vehículos son importantes.pdf'],
  ['2f22e410-5399-4727-9f36-8fcc687c132d', 'P6-L15-Cuánta energía produce el viento.pdf'],
  ['6a4960aa-8dc7-4cf1-8b93-2e673b1ba05c', 'P6-L16-Cuántas estructuras puedo armar.pdf'],
  ['b4691b3b-0200-4b43-b10e-065f1d0a681f', 'P6-L17-¡Movámonos!.pdf'],
  ['f1756ff5-480a-4f3b-93d8-0d8afc6dd804', 'P6-L18-¿Simple o compuesta.pdf'],
  ['52ab1521-34d1-4992-8555-3cfe3865a07e', 'P6-L19-Construyamos con engranajes.pdf'],
  ['25e10024-0d4a-4ccf-b9c0-9cda3167082f', 'P6-L20-Hablemos sobre medios de transporte.pdf'],
  ['cf924145-9141-4806-98c9-764dcb98b112', 'P6-L21-La energía que mueve mi cuerpo.pdf'],
  ['9902323a-19c7-49fc-9a0f-eb262fd44d19', 'P6-L22-¿Conoces este insecto.pdf']
]

S1 = [
  ['bc5afb4d-280b-49aa-b253-f25e1501ad77', 'S1-L01-Qué armados puedo realizar.pdf'],
  ['127d0cdc-7036-4c6f-926a-49bde4e22c56', 'S1-L02-Recordemos el principio de gravedad.pdf'],
  ['af002381-6545-442f-ba46-e191f7d2daba', 'S1-L03-Para qué sirven las balanzas.pdf'],
  ['916b3f08-e1a8-4628-8eaa-fff2c2346cc1', 'S1-L04-Aprendamos de máquinas simples.pdf'],
  ['5fb31404-6aed-4956-a93c-17419e0f3b77', 'S1-L05-Tiremos la basura en su lugar.pdf'],
  ['afcf27ef-56c2-47ff-adb4-01e511715a56', 'S1-L06-Creemos nuestros propios manuales.pdf'],
  ['1e0458cd-c802-4a9b-9b3e-409b5e00e6b0', 'S1-L07-Dejemos que Newton nos enseñe.pdf'],
  ['7d6f97e6-c379-4d5f-921b-50991802f685', 'S1-L08-Conozcamos sobre poleas.pdf'],
  ['35cf8472-9232-483e-9be0-067d2eddffec', 'S1-L09-Diseñemos con contrapesos.pdf'],
  ['4e12c6db-42ab-4a04-adcc-643deca121e8', 'S1-L10-Qué necesito para ser constructor.pdf'],
  ['c176350e-d58e-454e-bb79-afa116a6dbae', 'S1-L11-Aprendamos de mecánica.pdf'],
  ['b00fc35f-8f58-4a4d-b0f8-597960ce9f1d', 'S1-L12-Qué hacen los policías.pdf'],
  ['7e0bd1f3-ad94-46b0-b6d4-589104a631e5', 'S1-L13-Así se produce la energía eléctrica.pdf'],
  ['248f615d-d6ea-45b4-b6da-a2be116c0ec0', 'S1-L14-Conozcamos sobre la energía eólica.pdf'],
  ['e5025eae-7132-4d21-a11e-18b3d6c5fc1b', 'S1-L15- Ideemos y clasifiquemos estructuras.pdf'],
  ['a614438d-ee08-4f12-a664-de1f2121f993', 'S1-L16-Redactemos noticias.pdf'],
  ['1c14a231-5421-457e-9355-e0d7ba4b22b6', 'S1-L17-Cómo funcionan los engranajes.pdf'],
  ['674ca304-51d6-406e-b62a-514c900ca1ba', 'S1-L18-Conozcamos sobre energía potencial.pdf'],
  ['21b5bb57-82d1-4ec3-9676-c5f459cbb1e0', 'S1-L19-Por qué se necesita una caja de velocidades.pdf'],
  ['0745ccb6-89b6-4888-9151-0d8462e34330', 'S1-L20-El agua también produce energía.pdf'],
  ['a0f67962-e0e9-4b2a-9899-1b5261e94154', 'S1-L21-Inventemos una película.pdf'],
  ['9ae5f51c-cbfb-4ac6-90c1-826f61d5ee0e', 'S1-L22-Todos subamos al tren.pdf']
]

S2 = [
  ['3e1fbda1-b26d-4dc8-8e57-7eb13f0da1cd', 'S2-L01-Identifiquemos ejes de simetría.pdf'],
  ['1ca708d3-8f67-48ca-ad8e-1d49d81fe7c0', 'S2-L02-Conozcamos sobre insectos.pdf'],
  ['c8f1258e-7e3e-42cf-a443-b2b62ca23de4', 'S2-L03-Hagamos un torneo de básquetbol.pdf'],
  ['309584f1-29d1-4996-af6b-102155d8c9b8', 'S2-L04-Aprendamos sobre la Edad Media.pdf'],
  ['ec2b6fb7-ea0d-45d0-b8e5-1e12049f767b', 'S2-L05-Primera Ley de Newton.pdf'],
  ['70e628d6-2871-4abb-9cde-403c72de09bb', 'S2-L06-Para qué sirven los planos inclinados.pdf'],
  ['e003d178-d40d-4a5c-98ce-f5d0aa4083ee', 'S2-L07-Si cae es porque hay gravedad.pdf'],
  ['c699f55d-c9c6-4273-b99d-abc025a0c755', 'S2-L08-Estas son las poleas.pdf'],
  ['19b0b190-5af2-40c7-a44f-0156b223dd90', 'S2-L09-Aplico los principios de robótica.pdf'],
  ['11fc7684-c7c2-463c-ba24-02eea69b93f5', 'S2-L10-Prevención en carretera.pdf'],
  ['a093631f-b959-4cf9-afd4-193cc2116623', 'S2-L11-Principio mecánico de ruedas y ejes.pdf'],
  ['6c99ea75-7c21-4653-b607-8a73cd46589d', 'S2-L12-Mira cómo se mueve.pdf'],
  ['ef996652-68ec-4eb2-9f4a-0270ba712672', 'S2-L13-Empuja con fuerza.pdf'],
  ['78753f93-ae45-4574-963a-57e423b00bdf', 'S2-L14-Qué tipo de energía es esta.pdf'],
  ['c2e754e1-a715-4c84-8c90-6ed4c6472500', 'S2-L15-De dónde vienen los productos.pdf'],
  ['b20833e4-8946-4b56-a60c-5827f287630f', 'S2-L16-Grandes estructuras.pdf'],
  ['b4162da0-8ba0-445c-b284-c64fee803be9', 'S2-L17-Estos son los engranajes.pdf'],
  ['c6cdee23-63cf-4714-875c-328749de236b', 'S2-L18-Ejercitémonos.pdf'],
  ['533b665f-72b7-4f9a-82b0-b068da999c13', 'S2-L19-Simple o compuesta.pdf'],
  ['d3498949-2ac2-47ec-bc50-5c682ccc75dd', 'S2-L20-Cómo ocupamos los combustibles fósiles.pdf'],
  ['aa5b73ab-721c-4e3d-ba2d-1720e2d2dc58', 'S2-L21-La energía que el agua produce.pdf'],
  ['e8d7971c-47c9-4957-83da-837a56c61b28', 'S2-L22-Desenterremos los fósiles.pdf']
]

S3 = [
  ['1847b5a9-333b-4627-bd50-01bce915cd94', 'S3-L01-Hablemos de las grandes ciudades.pdf'],
  ['60250418-d82c-4e63-aee3-af38aa2af6be', 'S3-L02-Cómo funcionan mis manos.pdf'],
  ['9693da55-3945-4c6b-a88d-3b5cf7a9c137', 'S3-L03-Máquinas simples en vehículos de carga.pdf'],
  ['5c5ec81f-fab1-4828-9861-1ed91be617de', 'S3-L04-Aprendamos a redactar una noticia.pdf'],
  ['54e13267-7861-4a11-b4c8-2cba38cfd961', 'S3-L05-Qué es el tornillo de Arquímedes.pdf'],
  ['6ea33120-fbd4-40a4-8938-65753ece4167', 'S3-L06-Máquinas compuestas.pdf'],
  ['26e96fa9-2fc0-482a-9d8e-62d698f2d142', 'S3-L07-Usos del plano inclinado.pdf'],
  ['0b98712e-d623-446e-aa71-4896c4bc6412', 'S3-L08-Pesos y contrapesos.pdf'],
  ['2034a203-00ab-42c5-b422-0b0dd060a779', 'S3-L09-Así funcionan las poleas.pdf'],
  ['fcfa19a0-87af-417e-aebe-6b00e40de6c0', 'S3-L10-Otro medio de transporte.pdf'],
  ['6e5c935f-c569-4c42-bff5-62aadf2631b0', 'S3-L11-Drones-Una apuesta a la tecnología.pdf'],
  ['08af1a3c-00e3-437e-9157-c029ef7b92ec', 'S3-L12-Ejes y ruedas.pdf'],
  ['8a23b119-0bed-4d52-8767-078839fb735b', 'S3-L13-Organicemos una competencia.pdf'],
  ['a0331e88-5e7e-4c6d-a9c1-0a9d15bee15b', 'S3-L14-Estructuras de diferentes tamaños.pdf'],
  ['bc60ff96-2c07-4be4-83d4-d52531cb6de9', 'S3-L15-Armemos diseños más complejos.pdf'],
  ['dff41063-d524-4a0a-9bd8-758927f268e2', 'S3-L16-Los fósiles vuelven a la vida.pdf'],
  ['625581f5-4eee-4d5d-bdc9-0d1a9e69b389', 'S3-L17-Tipos de engranajes.pdf'],
  ['b3f6b488-71a0-4fa4-909e-27e68f4139a2', 'S3-L18-Terremotos y sismos.pdf'],
  ['36d425e1-5f7a-4d6e-89e6-93574cf9f03b', 'S3-L19-Reloj mundial.pdf'],
  ['2bd786b9-560f-4e55-9e3b-f2f6590f5b80', 'S3-L20-La marea alta.pdf'],
  ['0ef8f3c0-890b-4d80-ab86-6639cd98d86d', 'S3-L21-Grecia antigua.pdf'],
  ['e29bd7d3-18a3-4402-8dbd-f5adecad80fd', 'S3-L22-Ejercitémonos.pdf']
]

B1 = [
  ['d643a38e-ce92-48ac-8cc4-4167426592cb', 'B1-L01-Conozcamos sobre mitología grecolatina.pdf'],
  ['616b7db5-a204-4e99-992d-24e40ba7b54f', 'B1-L02-Maquinaria y modernización.pdf'],
  ['a097a78d-f262-4026-81b1-4e6f4f4ce393', 'B1-L03-Aprendamos de robótica.pdf'],
  ['b2815b85-a391-449c-8b1f-0415b547a38a', 'B1-L04-Máquinas simples_La palanca.pdf'],
  ['20bc8969-a4ae-4d32-9628-5018b6a87963', 'B1-L05-Uso planos inclinados.pdf'],
  ['2096ba1c-4bbb-4858-8cfd-bcd43d172a9c', 'B1-L06-Fuerzas paralelas.pdf'],
  ['cb212f5c-2340-4790-a4f3-4190ff5f056f', 'B1-L07-Aprendamos sobre gravedad.pdf'],
  ['caa597d6-811a-46ad-be63-c3a6b5b4ff1e', 'B1-L08-Participemos en la competencia.pdf'],
  ['bf5e3db3-edcf-45e1-b809-fdffae2b2055', 'B1-L09-Pesos y contrapesos.pdf'],
  ['0a152a8f-f2df-4f48-be4c-7d6e8965cd78', 'B1-L10-Armado y uso de poleas.pdf'],
  ['87275d78-6a0c-4403-860e-c9078fbd01ca', 'B1-L11-Calculemos la energía.pdf'],
  ['cc509c19-b6ce-437a-ad8b-370953595fb8', 'B1-L12-Las velocidades son relativas.pdf'],
  ['a173e015-4230-4c82-9e40-b21bd296ed5a', 'B1-L14-La evolución.pdf'],
  ['00f24182-8ce3-441e-b063-58c261090cd4', 'B1-L15-Estructuras.pdf'],
  ['2e5e8779-cc0d-49db-9e3b-3e0621fb0ded', 'B1-L16-Estructura de las moléculas.pdf'],
  ['edbe8db7-6426-43d4-ba38-643a16a08222', 'B1-L17-Tipos de engranajes.pdf'],
  ['1422ee0a-da7e-4775-af06-c3edd80009a0', 'B1-L18-Ensamblo máquinas compuestas.pdf'],
  ['a1680864-445a-4fc8-a4ca-574b1f927b71', 'B1-L19-Máquinas de producción.pdf'],
  ['2600dc6c-53d9-43a1-af33-e52347bd40d1', 'B1-L20-Segunda Ley de Newton.pdf'],
  ['b773fd58-ff3c-4a8f-98f5-3e87a2ed4be5', 'B1-L21-Esta es la energía hidráulica.pdf'],
  ['daa2b784-8170-47e1-a03f-753857e842b7', 'B1-L22-Una energía amigable con el medio ambiente.pdf']
]

B2 = [
  ['f4316e10-5a71-4986-9af9-f485f94b3060', 'B2-L01-Organicemos una competencia.pdf'],
  ['6403e049-b149-4cd0-ba2f-8f1bf01419a5', 'B2-L02-Qué tipo de palanca es esta.pdf'],
  ['26a10f52-21d1-487a-a3f7-620caa700751', 'B2-L03-Apliquemos la Ley de la palanca.pdf'],
  ['4fe6cc30-176a-440b-8717-c9196eccce71', 'B2-L04-Conozcamos el funcionamiento de nuestras manos.pdf'],
  ['ee3ac821-51d3-4224-8806-70767abe2431', 'B2-L05-Tornillo de Arquímedes.pdf'],
  ['20260d90-1ede-48cc-bf33-9fa7c3e9e26f', 'B2-L06-El plano inclinado y la fricción.pdf'],
  ['452b69e3-9c35-4912-902a-bfe48a061c32', 'B2-L07-De una simple a una compuesta.pdf'],
  ['1ed723f1-04f9-4fc4-b994-ac2b6a601b23', 'B2-L08-Usos y ventajas de las poleas.pdf'],
  ['bb92c818-83ad-4e71-8770-9ab35aca1324', 'B2-L09-Medios de transporte optativos.pdf'],
  ['3743b3dc-c036-42dd-b1f6-0563814e2b28', 'B2-L10-Aprendamos de mecánica automotriz.pdf'],
  ['ae70084d-d8d3-418f-b6de-a28f46120eca', 'B2-L11-Argumento sobre nuestras costumbres.pdf'],
  ['a0d603eb-1a3f-4fb2-a14b-c2ea745b0636', 'B2-L12-Ruedas y ejes.pdf'],
  ['855ffc3e-cc6a-4f43-9558-b3a3340a0cd4', 'B2-L13-Tecnología y desarrollo sostenible.pdf'],
  ['16a32864-9875-42e7-8d1f-7a7821fc4cc2', 'B2-L14-Observando ecosistemas imperceptibles.pdf'],
  ['b294681e-d324-445a-af07-819fda820077', 'B2-L15-Conozcamos de historia.pdf'],
  ['4bc92ec7-8f63-4c29-942c-de7ef5c05523', 'B2-L16-Usos de las estructuras.pdf'],
  ['02071909-7892-4d74-bdb6-8b00c5fe1abc', 'B2-L18-Viaje a la luna.pdf'],
  ['59c3ddfe-5ee4-4144-b238-858fcc013f52', 'B2-L19-Las construcciones, ¿perjudican el medio ambiente¿.pdf'],
  ['01ee0020-87e9-4227-a6d9-b544e934b8c5', 'B2-L20-Fuerza de fricción.pdf'],
  ['4281d866-cd00-4230-a50f-1060bbf7b211', 'B2-L21-Energía amigable con el medio ambiente.pdf'],
  ['3ec25aeb-ca61-4c96-b788-1b7312198456', 'B2-L22-De la Edad Antigua a la contemporaneidad.pdf']
]

A = [
  ['8d668666-9f8e-487c-9ff7-e5d5d6ea5531', 'EB1_L01-Fotorresistencia'],
  ['93d3c567-25c8-44d2-b163-6f072db1b785', 'EB1_L02-Control LED'],
  ['47f91c2b-9c40-447e-9c56-02a11139c6cf', 'EB1_L03-Luces con NeoPixels'],
  ['f1a7f157-99ba-44e7-a3c6-9074feba9454', 'EB1_L04-Control vehicular'],
  ['7fd69612-88f7-4712-a758-1383aa5cec94', 'EB1_L05-Color musical'],
  ['049048f5-3cb5-44a6-8ffd-f878a107d9d7', 'EB1_L06-Temporizador'],
  ['d57c52b6-4c57-493a-ab34-e0a4d1cd164c', 'EB1_L07-Ohmímetro'],
  ['eef2d120-2366-4ac3-a972-5b12eeb08873', 'EB1_L08-Sistema de enfriamiento'],
  ['ec46e47a-049c-4d6e-96f3-d10788a810d7', 'EB1_L09-Luces de aceleración'],
  ['d8d5bbd8-9ff1-4a80-889d-6ce09779d566', 'EB1_L10-Viaje seguro'],
  ['a168fc63-35a4-4740-8cf9-06246e7703d0', 'EB2_L01-Relé electromagnético'],
  ['fdd5d53a-768c-4a42-812a-b8ff7f3a0846', 'EB2_L02-Luces aceleradas'],
  ['55e51b77-26a7-41fa-9983-588f83cf7a8c', 'EB2_L03-Bucles de luces'],
  ['a0fd135e-c4ba-4676-a4c2-cee6f4566269', 'EB2_L04-Control de sonido'],
  ['2a67911c-34b0-47a3-8a97-a75e730fae02', 'EB2_L05-Música y color'],
  ['8c9c794d-38d2-4003-9d48-888e16cb0d58', 'EB2_L06-Programador de tareas'],
  ['05f63f77-ce4c-4008-8c5c-782ca67f119c', 'EB2_L07-Calculadora binaria'],
  ['f6812f9f-2a2c-48e1-ad01-e2797b351df3', 'EB2_L08-Luces inteligentes'],
  ['ad4a8feb-22e3-4bf7-8b6a-786f95490b43', 'EB2_L09-Velocidad y aceleración'],
  ['3ce82438-77ea-4f46-b5d4-dd619d702819', 'EB2_L10-Robot explorador'],
  ['91436973-1fe2-4432-9d93-2246b51f0b1d', 'ES1_L01-¿Qué es Arduino?'],
  ['0e4f54da-50b7-46c0-8a44-2317d566e90d', 'ES1_L02-Mi primer programa con Arduino'],
  ['b4a32202-8f39-4142-bace-90af91b630fe', 'ES1_L03-Ciclos repetitivos de programación'],
  ['9bc6137d-eac2-41e9-aabb-dd3ea9bc4fa8', 'ES1_L04-Programando un semáforo'],
  ['4e7dba34-1b91-4f01-a857-b22e0ca75f3c', 'ES1_L05-Principio de las pantallas LED'],
  ['c33f99f0-1e84-4e8c-9d22-083d25edb970', 'ES1_L06-Contador digital'],
  ['d8ad7518-1f6c-4bb8-99b5-5ebabb8e4849', 'ES1_L07-Luxómetro'],
  ['0da5a8ba-1108-4731-aca6-b838dfa8b190', 'ES1_L08-Termómetro digital'],
  ['1b7c04d8-a56f-43f5-9663-73ba89fa6dcb', 'ES1_L09-Movimientos automatizados'],
  ['d1e8cc1c-99aa-4339-b22b-a91f4089a386', 'ES1_L10-Robot musical'],
  ['a09c1311-74cf-4655-85e9-82c65cecaf12', 'ES2_L01-Programando microcontroladores'],
  ['4d1da65b-b254-44e4-a86f-acb7e301ee8a', 'ES2_L02-Comandos básicos Arduino'],
  ['2d4bdb16-cca5-405b-9dfb-0fa70d6d62a9', 'ES2_L03-Retardo LED'],
  ['dd439a13-f166-4a0f-aee2-fdd50aaca88c', 'ES2_L04-Cuidando el medio ambiente'],
  ['e0e17493-91b9-43f7-8600-f360e1f4ded0', 'ES2_L05-Combinando luz LED RGB'],
  ['31e00027-dc58-48ca-9caf-c9725ab9dccf', 'ES2_L06-Cronómetro digital'],
  ['6238282d-cfa4-47f2-b831-360943d05599', 'ES2_L07-Voltímetro digital'],
  ['ff81d490-3391-4739-926e-694fb48bad62', 'ES2_L08-Sonidos en mi habitación'],
  ['be436f0a-4f1f-4987-ac0d-3552f39a8dec', 'ES2_L09-Control de velocidad'],
  ['5b472be5-f5df-4d27-ab70-708c441897a1', 'ES2_L10-Robot policial'],
  ['479803ba-313e-4e11-99d7-9e1536eb4e62', 'ES3_L01-Circuitos integrados'],
  ['8e4c9784-88ef-4436-b4b2-951797308c31', 'ES3_L02-Programando Arduino'],
  ['83d9b2f1-f0b8-407d-916c-ebdd02632b1f', 'ES3_L03-Bucles y efectos luminosos'],
  ['c1694ffb-6287-4120-abdc-a51549365ecb', 'ES3_L04-Semáforo inteligente'],
  ['98acb9f0-43d8-46f8-bf3a-b0c94728ccf0', 'ES3_L05-Proyecto arcoiris'],
  ['3fc6bafa-717e-4e6f-85d6-3284ed49623a', 'ES3_L06-Reloj digital'],
  ['94f76f0f-820a-41a3-a4ff-74062f71f918', 'ES3_L07-Amperímetro digital'],
  ['6425927d-d9f8-44fd-9bd6-f83d5edd2ff7', 'ES3_L08-Sensor ultrasónico'],
  ['c71e9107-93e5-40c0-b6e1-bc2dad140faf', 'ES3_L09-Aceleración programada'],
  ['b79633d6-4111-4470-9119-b7e500b6dad5', 'ES3_L10-Robot mensajero']
]

M3D = [
  "2e2d45b9-3b30-476d-b432-d23c989fef48",
  "63658ba0-6139-4f9a-8cc3-0ace63277921",
  "8569d806-9034-4f69-9509-59bd64feb56a",
  "b471fb75-c4a1-445d-ba9c-182a83a0fc23",

  "e6d8fa4c-6753-4eda-9512-70e0bc0b9444",
  "12651df5-e694-4eeb-ae81-7f4027dbd329",
  "97edd512-7bad-4fd7-87e9-b61edf22dc95"
]


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

id_class_group.forEach(group_id => {
  // console.log(group_id);
  id_lesson.forEach(lesson_id => {
      // console.log(lesson_id)
        let cmd = `npm run aker-gql-op -- lesson_group_add  --lesson_id ${lesson_id} --group_id ${group_id}`;
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