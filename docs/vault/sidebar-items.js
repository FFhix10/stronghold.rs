initSidebarItems({"enum":[["Error","Errors for the Vault Crate"],["Kind","Enum that describes the type of a transaction."],["PreparedRead","A preparation of the data to read from the database."]],"mod":[["base64",""],["crypto_box",""],["types",""],["vault",""]],"static":[["ALLOC","A Zeroing Allocator which wraps the standard memory allocator. This allocator zeroes out memory when it is dropped. Works on any application that imports stronghold."]],"struct":[["ChainId","A chain identifier.  Used to identify a set of transactions in a version."],["DBReader","A reader for the `DBView`"],["DBView","A view over the records in a vault.  Allows for loading and accessing the records."],["DBWriter","A writer for the `DBView`.  Allows mutation of the underlying data."],["DeleteRequest","A delete a transaction Request. Contains the id and Kind of the transaction to be revoked."],["Key","A key to the crypto box.  Key is stored on the heap which makes it easier to erase."],["ReadRequest","A read request call.  Contains the ID and Kind for the referred transaction."],["ReadResult","A Read Result which contains the data from the read request.  Contains the Kind, id and data of the transaction. The data is guarded and needs to be unlocked and decrypted before it can be used."],["RecordHint","a record hint.  Used as a hint to what this data is."],["RecordId","A record identifier.  Contains a ChainID which refers to the \"chain\" of transactions in the Version."],["WriteRequest","A Write Request.  Contains the id, kind and data of the transaction that will be mutated."]],"trait":[["Base64Decodable","a trait to make types base64 decodable"],["Base64Encodable","a trait to make types base64 encodable"],["BoxProvider","A provider interface between the vault and a crypto box. See libsodium's secretbox for an example."],["Decrypt","Trait for decryptable data.  Allows the data to be decrypted."],["Encrypt","trait for encryptable data. Allows the data to be encrypted."]],"type":[["Result",""]]});