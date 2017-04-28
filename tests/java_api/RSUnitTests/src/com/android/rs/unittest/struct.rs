/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "shared.rsh"

typedef struct Point2 {
   int x;
   int y;
} Point_2;
Point_2 *point2;

static bool test_Point_2(int expected) {
    bool failed = false;

    rsDebug("Point: ", point2[0].x, point2[0].y);
    _RS_ASSERT(point2[0].x == expected);
    _RS_ASSERT(point2[0].y == expected);

    if (failed) {
        rsDebug("test_Point_2 FAILED", 0);
    }
    else {
        rsDebug("test_Point_2 PASSED", 0);
    }

    return failed;
}

void struct_test(int expected) {
    bool failed = false;
    failed |= test_Point_2(expected);

    if (failed) {
        rsSendToClientBlocking(RS_MSG_TEST_FAILED);
    }
    else {
        rsSendToClientBlocking(RS_MSG_TEST_PASSED);
    }
}
