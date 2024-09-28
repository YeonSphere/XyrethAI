import 'dart:ffi';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

class SeokjinAI {
  static final SeokjinAI _instance = SeokjinAI._internal();
  late final SeokjinAIImpl _impl;

  factory SeokjinAI() {
    return _instance;
  }

  SeokjinAI._internal() {
    _impl = SeokjinAIImpl(loadDylib());
  }

  Future<String> processInput(String input) => _impl.processInput(input);
  
  Future<void> loadContextFromFile(String filePath) => _impl.loadContextFromFile(filePath);
  
  Future<void> learnFromWeb(String startUrl) => _impl.learnFromWeb(startUrl);
}

class SeokjinAIImpl {
  final SeokjinAIBridge _bridge;

  SeokjinAIImpl(DynamicLibrary dylib) : _bridge = SeokjinAIBridge(dylib);

  Future<String> processInput(String input) => _bridge.processInput(input);
  Future<void> loadContextFromFile(String filePath) => _bridge.loadContextFromFile(filePath);
  Future<void> learnFromWeb(String startUrl) => _bridge.learnFromWeb(startUrl);
}

class SeokjinAIBridge {
  final _processInput = FlutterRustBridgeTask<String, String>();
  final _loadContextFromFile = FlutterRustBridgeTask<void, String>();
  final _learnFromWeb = FlutterRustBridgeTask<void, String>();

  SeokjinAIBridge(DynamicLibrary dylib) {
    _processInput.init(dylib.lookupFunction<NativeString Function(Pointer<Utf8>), String Function(Pointer<Utf8>)>('process_input'));
    _loadContextFromFile.init(dylib.lookupFunction<Void Function(Pointer<Utf8>), void Function(Pointer<Utf8>)>('load_context_from_file'));
    _learnFromWeb.init(dylib.lookupFunction<Void Function(Pointer<Utf8>), void Function(Pointer<Utf8>)>('learn_from_web'));
  }

  Future<String> processInput(String input) => _processInput.run(input);
  Future<void> loadContextFromFile(String filePath) => _loadContextFromFile.run(filePath);
  Future<void> learnFromWeb(String startUrl) => _learnFromWeb.run(startUrl);
}

DynamicLibrary loadDylib() {
  final dylib = DynamicLibrary.open('libseokjin_ai.so');
  return dylib;
}
