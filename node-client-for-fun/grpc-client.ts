const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');
const packageDefinition = protoLoader.loadSync('../proto/todo.proto', {});
const todoPackage = grpc.loadPackageDefinition(packageDefinition).todo;

export const todoClient = new todoPackage.Todo('localhost:8080', grpc.credentials.createInsecure());