/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

[Constructor(optional UnderlyingSource underlyingSource, optional QueuingStrategy strategy),
 Exposed=(Window,Worker)]
interface ReadableStream {
  readonly attribute boolean locked;
  Promise<void> cancel(optional any reason);
  [Throws] ReadableStreamDefaultReader getReader();
  [Throws] sequence<ReadableStream> tee();
};

callback ReadableStreamDefaultControllerCallback = any (ReadableStreamDefaultController controller);
callback ReadableStreamErrorCallback = any (any reason);

dictionary UnderlyingSource {
  ReadableStreamDefaultControllerCallback? start;
  ReadableStreamDefaultControllerCallback? pull;
  ReadableStreamErrorCallback? cancel;
};

[Exposed=(Window,Worker)]
interface ReadableStreamDefaultController {
  readonly attribute unrestricted double? desiredSize;

  [Throws] void close();
  [Throws] void enqueue(any chunk);
  void error(optional any error);
};

interface ReadableStreamReadResult {
  readonly attribute boolean done;
  readonly attribute any value;
};

[Exposed=(Window,Worker)]
interface ReadableStreamDefaultReader {
  readonly attribute Promise<void> closed;

  Promise<void> cancel(optional any reason);
  Promise<ReadableStreamReadResult> read();
  [Throws] void releaseLock();
};
