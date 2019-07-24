/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

#pragma once
#include <src/gen-cpp2/MyRoot.h>
#include <src/gen-cpp2/MyNode.h>
#include <src/gen-cpp2/MyLeaf.h>

#include <folly/futures/Future.h>
#include <folly/futures/Promise.h>
#include <folly/Unit.h>
#include <thrift/lib/py3/clientcallbacks.h>
#include <thrift/lib/py3/client_wrapper.h>

#include <cstdint>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include <vector>

namespace cpp2 {

class MyRootClientWrapper : public ::thrift::py3::ClientWrapper {
  public:
    explicit MyRootClientWrapper(
      std::shared_ptr<::cpp2::MyRootAsyncClient> async_client,
      std::shared_ptr<apache::thrift::RequestChannel> channel);

    folly::Future<folly::Unit> do_root(
      apache::thrift::RpcOptions& rpcOptions);
};


class MyNodeClientWrapper : public ::cpp2::MyRootClientWrapper {
  public:
    explicit MyNodeClientWrapper(
      std::shared_ptr<::cpp2::MyNodeAsyncClient> async_client,
      std::shared_ptr<apache::thrift::RequestChannel> channel);

    folly::Future<folly::Unit> do_mid(
      apache::thrift::RpcOptions& rpcOptions);
};


class MyLeafClientWrapper : public ::cpp2::MyNodeClientWrapper {
  public:
    explicit MyLeafClientWrapper(
      std::shared_ptr<::cpp2::MyLeafAsyncClient> async_client,
      std::shared_ptr<apache::thrift::RequestChannel> channel);

    folly::Future<folly::Unit> do_leaf(
      apache::thrift::RpcOptions& rpcOptions);
};


} // namespace cpp2
