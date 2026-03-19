// graphql operation as it appears in graphql file
const grapqql_operation = 'robot_add'

// receive arguments and turn them into json variables
const argv = require('minimist')(process.argv.slice(2))
var variables = argv
delete variables._
console.log("variables", variables)

// load config file: .graphqlrc
const { loadConfigSync } = require('graphql-config')
const config = loadConfigSync({})
const api = config.getDefault().extension("endpoints")["dev"]
console.log("api", api)

// find graphql operation inside config
const jp = require('jsonpath')
let docs = config.getDefault().getDocumentsSync()
let operation = jp.query(docs, '$[*].document.definitions[?(@.name.value == "' + grapqql_operation + '")]')[0]
console.log("operation", operation)

// process graphql request
const { request, gql, GraphQLClient } = require('graphql-request')
async function main() {
  const endpoint = api.url
  const graphQLClient = new GraphQLClient(endpoint, { headers: api.headers })
  const data = await graphQLClient.request(operation, variables)
  console.log(JSON.stringify(data, undefined, 2))
}
main().catch((error) => console.error(error))
