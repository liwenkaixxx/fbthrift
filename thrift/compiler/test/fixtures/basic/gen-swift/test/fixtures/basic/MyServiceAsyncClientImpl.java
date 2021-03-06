/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

package test.fixtures.basic;

import com.facebook.nifty.client.RequestChannel;
import com.facebook.swift.codec.*;
import com.facebook.swift.service.*;
import com.facebook.swift.service.metadata.*;
import com.facebook.swift.transport.client.*;
import com.google.common.util.concurrent.ListenableFuture;
import java.io.*;
import java.lang.reflect.Method;
import java.util.*;
import org.apache.thrift.ProtocolId;
import reactor.core.publisher.Mono;

@SwiftGenerated
public class MyServiceAsyncClientImpl extends AbstractThriftClient implements MyService.Async {

    // Method Handlers
    private ThriftMethodHandler pingMethodHandler;
    private ThriftMethodHandler getRandomDataMethodHandler;
    private ThriftMethodHandler hasDataByIdMethodHandler;
    private ThriftMethodHandler getDataByIdMethodHandler;
    private ThriftMethodHandler putDataByIdMethodHandler;
    private ThriftMethodHandler lobDataByIdMethodHandler;

    // Method Exceptions
    private static final Class[] pingExceptions = new Class[] {
        org.apache.thrift.TException.class};
    private static final Class[] getRandomDataExceptions = new Class[] {
        org.apache.thrift.TException.class};
    private static final Class[] hasDataByIdExceptions = new Class[] {
        org.apache.thrift.TException.class};
    private static final Class[] getDataByIdExceptions = new Class[] {
        org.apache.thrift.TException.class};
    private static final Class[] putDataByIdExceptions = new Class[] {
        org.apache.thrift.TException.class};
    private static final Class[] lobDataByIdExceptions = new Class[] {
        org.apache.thrift.TException.class};

    public MyServiceAsyncClientImpl(
        RequestChannel channel,
        Map<Method, ThriftMethodHandler> methods,
        Map<String, String> headers,
        Map<String, String> persistentHeaders,
        List<? extends ThriftClientEventHandler> eventHandlers) {
      super(channel, headers, persistentHeaders, eventHandlers);

      Map<String, ThriftMethodHandler> methodHandlerMap = new HashMap<>();
      methods.forEach(
          (key, value) -> {
            methodHandlerMap.put(key.getName(), value);
          });

      // Set method handlers
      pingMethodHandler = methodHandlerMap.get("ping");
      getRandomDataMethodHandler = methodHandlerMap.get("getRandomData");
      hasDataByIdMethodHandler = methodHandlerMap.get("hasDataById");
      getDataByIdMethodHandler = methodHandlerMap.get("getDataById");
      putDataByIdMethodHandler = methodHandlerMap.get("putDataById");
      lobDataByIdMethodHandler = methodHandlerMap.get("lobDataById");
    }

    public MyServiceAsyncClientImpl(
        Map<String, String> headers,
        Map<String, String> persistentHeaders,
        Mono<? extends RpcClient> rpcClient,
        ThriftServiceMetadata serviceMetadata,
        ThriftCodecManager codecManager,
        ProtocolId protocolId,
        Map<Method, ThriftMethodHandler> methods) {
      super(headers, persistentHeaders, rpcClient, serviceMetadata, codecManager, protocolId);

      Map<String, ThriftMethodHandler> methodHandlerMap = new HashMap<>();
      methods.forEach(
          (key, value) -> {
            methodHandlerMap.put(key.getName(), value);
          });

      // Set method handlers
      pingMethodHandler = methodHandlerMap.get("ping");
      getRandomDataMethodHandler = methodHandlerMap.get("getRandomData");
      hasDataByIdMethodHandler = methodHandlerMap.get("hasDataById");
      getDataByIdMethodHandler = methodHandlerMap.get("getDataById");
      putDataByIdMethodHandler = methodHandlerMap.get("putDataById");
      lobDataByIdMethodHandler = methodHandlerMap.get("lobDataById");
    }

    @java.lang.Override
    public void close() {
        super.close();
    }


    @java.lang.Override
    public ListenableFuture<Void> ping() {
        try {
          return (ListenableFuture<Void>) execute(pingMethodHandler, pingExceptions);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    @java.lang.Override
    public ListenableFuture<String> getRandomData() {
        try {
          return (ListenableFuture<String>) execute(getRandomDataMethodHandler, getRandomDataExceptions);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    @java.lang.Override
    public ListenableFuture<Boolean> hasDataById(
        long id) {
        try {
          return (ListenableFuture<Boolean>) execute(hasDataByIdMethodHandler, hasDataByIdExceptions, id);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    @java.lang.Override
    public ListenableFuture<String> getDataById(
        long id) {
        try {
          return (ListenableFuture<String>) execute(getDataByIdMethodHandler, getDataByIdExceptions, id);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    @java.lang.Override
    public ListenableFuture<Void> putDataById(
        long id,
        String data) {
        try {
          return (ListenableFuture<Void>) execute(putDataByIdMethodHandler, putDataByIdExceptions, id, data);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    @java.lang.Override
    public ListenableFuture<Void> lobDataById(
        long id,
        String data) {
        try {
          return (ListenableFuture<Void>) execute(lobDataByIdMethodHandler, lobDataByIdExceptions, id, data);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }


    public ListenableFuture<Void> ping(
        RpcOptions rpcOptions) {
        try {
          return (ListenableFuture<Void>) executeWithOptions(pingMethodHandler, pingExceptions, rpcOptions);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    public ListenableFuture<String> getRandomData(
        RpcOptions rpcOptions) {
        try {
          return (ListenableFuture<String>) executeWithOptions(getRandomDataMethodHandler, getRandomDataExceptions, rpcOptions);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    public ListenableFuture<Boolean> hasDataById(
        long id,
        RpcOptions rpcOptions) {
        try {
          return (ListenableFuture<Boolean>) executeWithOptions(hasDataByIdMethodHandler, hasDataByIdExceptions, rpcOptions, id);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    public ListenableFuture<String> getDataById(
        long id,
        RpcOptions rpcOptions) {
        try {
          return (ListenableFuture<String>) executeWithOptions(getDataByIdMethodHandler, getDataByIdExceptions, rpcOptions, id);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    public ListenableFuture<Void> putDataById(
        long id,
        String data,
        RpcOptions rpcOptions) {
        try {
          return (ListenableFuture<Void>) executeWithOptions(putDataByIdMethodHandler, putDataByIdExceptions, rpcOptions, id, data);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }

    public ListenableFuture<Void> lobDataById(
        long id,
        String data,
        RpcOptions rpcOptions) {
        try {
          return (ListenableFuture<Void>) executeWithOptions(lobDataByIdMethodHandler, lobDataByIdExceptions, rpcOptions, id, data);
        } catch (Throwable t) {
          throw new RuntimeTException(t.getMessage(), t);
        }
    }
}
