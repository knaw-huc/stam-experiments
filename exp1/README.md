# Experiment 1: STAM Tools on a very small text

This experiment invokes most of the [STAM Tools](https://github.com/annotation/stam-tools) as available at the time of writing.

It demonstrates:

1. `stam init`: initializing a stam annotation store from scratch, adding only a single small input text
2. `stam info`: info on the annotation store (containing nothing but the text resource at this point)
3. `stam tag`: run a simple regular-expression based tagger on the resources, effectively tokenizing it and producing STAM data. The tagging is on on the basis of [these simple rules](../config/stam-tag/simpletagger.tsv).
4. `stam validate`: validate all of the resulting STAM JSON
5. `stam info`: info on the annotation store at this point
6. `stam to-tsv`: output the annotations in the store to tsv 

Please inspect the [Makefile](Makefile) for details.

## Usage

Just run ``make`` to run the experiment (``make clean`` to remove results again)

You may want to run this in the provided Docker container (one level up).

## Output

```
STAGE 1/6 ---------------------
stam init --resource smallquote.txt exp1.store.stam.json
Initializing store with 0 annotation(s), 1 resource(s), 0 annotationset(s), 0 additional store(s)


STAGE 2/6-------------------------------
stam info exp1.store.stam.json
Loading annotation store exp1.store.stam.json
(Tip: add --verbose for more detailed info output)
Configuration: Config { textrelationmap: true, resource_annotation_map: true, dataset_annotation_map: true, annotation_annotation_map: true, generate_ids: true, use_include: true, workdir: None, debug: false, serialize_mode: RwLock { data: AllowInclude, poisoned: false, .. } }
Filename: "exp1.store.stam.json"
Indices:
    - dataset_data_annotation_map:      0
    - textrelationmap:                  0
    - resource_annotation_map:          0
    - dataset_annotation_map:           0
    - annotation_annotation_map:        0
Resources:              1
    - [0] Resource ID: "smallquote.txt"; textlength: 77
Annotation datasets:    0
Annotations:            0


STAGE 3/6 -------------------------------
stam tag --rules simpletagger.tsv exp1.store.stam.json
Loaded 3 expressions from simpletagger.tsv


STAGE 4/6 -------------------------------
stam validate --verbose exp1.store.stam.json | jq
Loading annotation store exp1.store.stam.json
```

```json
{
  "@type": "AnnotationStore",
  "resources": [
    {
      "@type": "TextResource",
      "@include": "smallquote.txt"
    }
  ],
  "annotationsets": [
    {
      "@type": "AnnotationDataSet",
      "@id": "simpletokens",
      "keys": [
        {
          "@type": "DataKey",
          "@id": "type"
        }
      ],
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "key": "type",
          "value": {
            "@type": "String",
            "value": "word"
          }
        },
        {
          "@type": "AnnotationData",
          "@id": "DZLYAPaFGHqa_cUTAckBKR",
          "key": "type",
          "value": {
            "@type": "String",
            "value": "punctuation"
          }
        }
      ]
    }
  ],
  "annotations": [
    {
      "@type": "Annotation",
      "@id": "A37TsnwDg7pCWUdycKCDQ8",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 0
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 1
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "Ac66B6AwVRoBxIE4Y1JExk",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 2
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 6
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AJcZXdPTbNh6dnh19-7t8F",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 7
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 9
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AQtg_IPfDranj_LrmNSwSg",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 10
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 17
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AXGjPvH-6_eP5185cN7oDy",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 18
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 24
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "ApFtASech_uXYejJJDJ6gp",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 26
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 27
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AHcxjmEK_VRV6yNCtEzULT",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 28
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 30
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AX79ZWuxtRHs6Llcn8ZrLt",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 31
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 35
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AGcP_Wfq_g615ACnhqW0kI",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 36
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 48
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AKecxi3WaNrDFkjWIQgQFH",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 49
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 56
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AisTS0qy3hBonuw1Pynfxi",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 61
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 67
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AjKC4wJ14M2Dn4R8Y_gbNo",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 68
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 76
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DyQJbC9JLQ05ciMNbfwAJC",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AtKkL65NiDXBzk5o3nNU_6",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 24
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 25
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DZLYAPaFGHqa_cUTAckBKR",
          "set": "simpletokens"
        }
      ]
    },
    {
      "@type": "Annotation",
      "@id": "AU4iUaA9NkexD_Q7mqy16Q",
      "target": {
        "@type": "TextSelector",
        "resource": "smallquote.txt",
        "offset": {
          "@type": "Offset",
          "begin": {
            "@type": "BeginAlignedCursor",
            "value": 56
          },
          "end": {
            "@type": "BeginAlignedCursor",
            "value": 57
          }
        }
      },
      "data": [
        {
          "@type": "AnnotationData",
          "@id": "DZLYAPaFGHqa_cUTAckBKR",
          "set": "simpletokens"
        }
      ]
    }
  ]
}
```
```
STAGE 5/6 -------------------------------
stam info --verbose exp1.store.stam.json
Loading annotation store exp1.store.stam.json
Configuration: Config { textrelationmap: true, resource_annotation_map: true, dataset_annotation_map: true, annotation_annotation_map: true, generate_ids: true, use_include: true, workdir: None, debug: false, serialize_mode: RwLock { data: AllowInclude, poisoned: false, .. } }
Filename: "exp1.store.stam.json"
Indices:
    - dataset_data_annotation_map:      14
    - textrelationmap:                  14
    - resource_annotation_map:          0
    - dataset_annotation_map:           0
    - annotation_annotation_map:        0
Resources:              1
    - [0] Resource ID: "smallquote.txt"; textlength: 77
        - [0] TextSelection; begin: 0; end: 1, text: "I", #annotations: 1
        - [1] TextSelection; begin: 2; end: 6, text: "have", #annotations: 1
        - [2] TextSelection; begin: 7; end: 9, text: "no", #annotations: 1
        - [3] TextSelection; begin: 10; end: 17, text: "special", #annotations: 1
        - [4] TextSelection; begin: 18; end: 24, text: "talent", #annotations: 1
        - [5] TextSelection; begin: 26; end: 27, text: "I", #annotations: 1
        - [6] TextSelection; begin: 28; end: 30, text: "am", #annotations: 1
        - [7] TextSelection; begin: 31; end: 35, text: "only", #annotations: 1
        - [8] TextSelection; begin: 36; end: 48, text: "passionately", #annotations: 1
        - [9] TextSelection; begin: 49; end: 56, text: "curious", #annotations: 1
        - [10] TextSelection; begin: 61; end: 67, text: "Albert", #annotations: 1
        - [11] TextSelection; begin: 68; end: 76, text: "Einstein", #annotations: 1
        - [12] TextSelection; begin: 24; end: 25, text: ".", #annotations: 1
        - [13] TextSelection; begin: 56; end: 57, text: ".", #annotations: 1
Annotation datasets:    1
    - [0] Set ID: "simpletokens"; #keys: 1; #data: 2
        - [0] Key ID: "type"; #data: 2
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Key: "type"; Value: String("word"); #annotations: 12
        - [1] Data ID: "DZLYAPaFGHqa_cUTAckBKR"; Key: "type"; Value: String("punctuation"); #annotations: 2
Annotations:            14
    - [0] Annotation ID: "A37TsnwDg7pCWUdycKCDQ8"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(0), end: BeginAligned(1) }); text: ["I"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [1] Annotation ID: "Ac66B6AwVRoBxIE4Y1JExk"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(2), end: BeginAligned(6) }); text: ["have"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [2] Annotation ID: "AJcZXdPTbNh6dnh19-7t8F"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(7), end: BeginAligned(9) }); text: ["no"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [3] Annotation ID: "AQtg_IPfDranj_LrmNSwSg"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(10), end: BeginAligned(17) }); text: ["special"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [4] Annotation ID: "AXGjPvH-6_eP5185cN7oDy"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(18), end: BeginAligned(24) }); text: ["talent"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [5] Annotation ID: "ApFtASech_uXYejJJDJ6gp"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(26), end: BeginAligned(27) }); text: ["I"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [6] Annotation ID: "AHcxjmEK_VRV6yNCtEzULT"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(28), end: BeginAligned(30) }); text: ["am"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [7] Annotation ID: "AX79ZWuxtRHs6Llcn8ZrLt"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(31), end: BeginAligned(35) }); text: ["only"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [8] Annotation ID: "AGcP_Wfq_g615ACnhqW0kI"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(36), end: BeginAligned(48) }); text: ["passionately"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [9] Annotation ID: "AKecxi3WaNrDFkjWIQgQFH"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(49), end: BeginAligned(56) }); text: ["curious"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [10] Annotation ID: "AisTS0qy3hBonuw1Pynfxi"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(61), end: BeginAligned(67) }); text: ["Albert"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [11] Annotation ID: "AjKC4wJ14M2Dn4R8Y_gbNo"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(68), end: BeginAligned(76) }); text: ["Einstein"], #data: 1
        - [0] Data ID: "DyQJbC9JLQ05ciMNbfwAJC"; Set ID: "simpletokens"; Key: "type"; Value: String("word")
    - [12] Annotation ID: "AtKkL65NiDXBzk5o3nNU_6"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(24), end: BeginAligned(25) }); text: ["."], #data: 1
        - [1] Data ID: "DZLYAPaFGHqa_cUTAckBKR"; Set ID: "simpletokens"; Key: "type"; Value: String("punctuation")
    - [13] Annotation ID: "AU4iUaA9NkexD_Q7mqy16Q"; target: TextSelector(TextResourceHandle(0), Offset { begin: BeginAligned(56), end: BeginAligned(57) }); text: ["."], #data: 1
        - [1] Data ID: "DZLYAPaFGHqa_cUTAckBKR"; Set ID: "simpletokens"; Key: "type"; Value: String("punctuation")


STAGE 6/6 -------------------------------
stam to-tsv --verbose exp1.store.stam.json
Loading annotation store exp1.store.stam.json
A37TsnwDg7pCWUdycKCDQ8	simpletokens	type	word	I	smallquote.txt#0-1
Ac66B6AwVRoBxIE4Y1JExk	simpletokens	type	word	have	smallquote.txt#2-6
AJcZXdPTbNh6dnh19-7t8F	simpletokens	type	word	no	smallquote.txt#7-9
AQtg_IPfDranj_LrmNSwSg	simpletokens	type	word	special	smallquote.txt#10-17
AXGjPvH-6_eP5185cN7oDy	simpletokens	type	word	talent	smallquote.txt#18-24
ApFtASech_uXYejJJDJ6gp	simpletokens	type	word	I	smallquote.txt#26-27
AHcxjmEK_VRV6yNCtEzULT	simpletokens	type	word	am	smallquote.txt#28-30
AX79ZWuxtRHs6Llcn8ZrLt	simpletokens	type	word	only	smallquote.txt#31-35
AGcP_Wfq_g615ACnhqW0kI	simpletokens	type	word	passionately	smallquote.txt#36-48
AKecxi3WaNrDFkjWIQgQFH	simpletokens	type	word	curious	smallquote.txt#49-56
AisTS0qy3hBonuw1Pynfxi	simpletokens	type	word	Albert	smallquote.txt#61-67
AjKC4wJ14M2Dn4R8Y_gbNo	simpletokens	type	word	Einstein	smallquote.txt#68-76
AtKkL65NiDXBzk5o3nNU_6	simpletokens	type	punctuation	.	smallquote.txt#24-25
AU4iUaA9NkexD_Q7mqy16Q	simpletokens	type	punctuation	.	smallquote.txt#56-57
```
