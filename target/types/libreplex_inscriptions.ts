export type LibreplexInscriptions = {
  "version": "0.1.1",
  "name": "libreplex_inscriptions",
  "instructions": [
    {
      "name": "createInscription",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "root",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "ordinal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ordinalInput",
          "type": {
            "defined": "CreateInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "appendToInscription",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscription",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "AppendToInscriptionInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "inscription",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "root",
            "type": "publicKey"
          },
          {
            "name": "dataLengthCurrent",
            "type": "u32"
          },
          {
            "name": "dataLengthMax",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "AppendToInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "appendData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "CreateInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxDataLength",
            "type": "u32"
          },
          {
            "name": "initialData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "InscriptionEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Append"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "InscriptionEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "InscriptionEventType"
          },
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6001,
      "name": "MaxSizeExceeded",
      "msg": "Max size exceeded"
    }
  ]
};

export const IDL: LibreplexInscriptions = {
  "version": "0.1.1",
  "name": "libreplex_inscriptions",
  "instructions": [
    {
      "name": "createInscription",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "root",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "ordinal",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "ordinalInput",
          "type": {
            "defined": "CreateInscriptionInput"
          }
        }
      ]
    },
    {
      "name": "appendToInscription",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "inscription",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "AppendToInscriptionInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "inscription",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "root",
            "type": "publicKey"
          },
          {
            "name": "dataLengthCurrent",
            "type": "u32"
          },
          {
            "name": "dataLengthMax",
            "type": "u32"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "AppendToInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "appendData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "CreateInscriptionInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxDataLength",
            "type": "u32"
          },
          {
            "name": "initialData",
            "type": "bytes"
          }
        ]
      }
    },
    {
      "name": "InscriptionEventType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "Create"
          },
          {
            "name": "Append"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "InscriptionEvent",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "eventType",
          "type": {
            "defined": "InscriptionEventType"
          },
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "BadAuthority",
      "msg": "Bad authority"
    },
    {
      "code": 6001,
      "name": "MaxSizeExceeded",
      "msg": "Max size exceeded"
    }
  ]
};