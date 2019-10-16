/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

callback QueuingStrategySizeCallback = double (any chunk);

dictionary QueuingStrategy {
  unrestricted double? highWaterMark;
  QueuingStrategySizeCallback? size;
};

dictionary QueuingStrategyOptions {
  unrestricted double highWaterMark;
};

[Constructor(QueuingStrategyOptions options),
 Exposed=(Window,Worker)]
interface ByteLengthQueuingStrategy: QueuingStrategy {
  attribute unrestricted double highWaterMark;
  double size(ArrayBufferView chunk);
};

[Constructor(QueuingStrategyOptions options),
 Exposed=(Window,Worker)]
interface CountQueuingStrategy: QueuingStrategy {
  attribute unrestricted double highWaterMark;
  double size(any chunk);
};
