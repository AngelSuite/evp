%%%
title = "Evidence Package Format Specification: Storing Evidence from Software Testing"
abbrev = "evp-spec"
ipr = "trust200902"
area = ""
workgroup = ""
keyword = ["evp", "evidence", "format", "specification"]
submissionType = "independent"

[seriesInfo]
name = "Internet-Draft"
value = "draft-hopkins-evp-spec-07"
stream = "independent"
status = "informational"

date = 2025-12-06T00:00:00Z

[[author]]
initials="L."
surname="Hopkins"
fullname="Lily Hopkins"
  [author.address]
  uri = "https://github.com/lilopkins"
  email = "lily@hpkns.uk"

[[author]]
inials="E."
surname="Turner"
fullname="Eden Turner"
  [author.address]
  uri = "https://github.com/Some-Birb7190"
  email = "somebirb7190@gmail.com"
%%%

.# Abstract

Taking evidence is a key part of any robust software testing process.
This specification defines a format which collects evidence together
and stores metadata and annotations in an organised fashion from both
manual and automated testing sources.

This work is not a standard and does not enjoy community consensus.

{mainmatter}

# Introduction

## Purpose

The purpose of this specification is to define a format for storage of
evidence produced as the result of software testing that:

* allows for basic collation of evidence;
* can store any kind of file type that might be produced;
* stores data compressed;
* stores related evidence together, but allows for dividing up by test
  case;
* allows test evidence to be attested with a tracable list of attestors,
  and;
* is built upon widely available standards.

The format does not attempt to:

* act as an captioned archiving solution for other purposes outside of
  software testing, even if it may be suitable for them.

## Intended Audience

This specification is intended for those who might wish to write their
own implementation of the evidence package format. There are a number of
situations where writing an implementation may be desirable:

* in an automation tool that runs a number of operations to
  automatically test something, to produce an evidence package
  containing the results of the automated testing;
* in a manual evidence collection tool, where a user might want to
  collect evidence in a single, easy to manage place for later
  processing or sharing;
* in an analysis tool, to view, annotate, share and understand the
  evidence from previous testing;
* in a viewer, to view evidence that has been shared, for example from a
  testing team to a customer, or;
* any other situation where it may be desirable to collect test evidence
  and bundle it together for later.

## Changes from Previous Versions

This document forms the original accepted specification.

# Terminology

The key words "**MUST**", "**MUST NOT**", "**REQUIRED**", "**SHALL**",
"**SHALL NOT**", "**SHOULD**", "**SHOULD NOT**", "**RECOMMENDED**",
"**NOT RECOMMENDED**", "**MAY**", and "**OPTIONAL**" in this document
are to be interpreted as described in BCP 14 [@!RFC2119] [@!RFC8174]
when, and only when, they appear in all capitals, as shown here.

# Specification

An evidence package is a structured ZIP archive [@!zip] using deflate
compression. It **MUST** contain the file "manifest.json", and the
directories "media" and "test_cases" internally within the ZIP archive.
This structure does not need to be represented outside of the ZIP
archive and as such the internal structure does not need to be
understood by an end-user of any tool that works with evidence packages.

<reference anchor="zip" target="https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT">
    <front>
        <title>.ZIP File Format Specification</title>
        <author>
            <organization>PKWARE, Inc.</organization>
        </author>
        <date year="2022" month="November" day="01"/>
    </front>
</reference>

See (#example-archive) for an example of the file's internal structure.

## "manifest.json" File

The manifest.json file defines metadata relating to the entire package
of evidence. It **MUST** be a UTF-8 encoded, LF line ended, JSON
[@!RFC8259] file with the following elements:

| Element    | Condition | Type | Section | Description |
|------------|-----------|------|---|---|
| $schema    | Optional  | String | (#manifest-schema) | The $schema element **MAY** point to a copy of the schema for the manifest. |
| metadata   | Mandatory | Object | (#manifest-metadata) | The metadata element stores package metadata. |
| custom_metadata | Mandatory | Object | (#manifest-custom-metadata) | Custom metadata fields for test cases in this package. |
| media      | Mandatory | Array | (#manifest-media) | The media element stores a list of media files that are stored in this evidence package. |
| test_cases | Mandatory | Array | (#manifest-test-cases) | The test_cases element stores a list of test cases. |

See an example manifest.json file in (#example-manifest).

### "$schema" Element {#manifest-schema}

This element **MAY** optionally be provided to point to a JSON schema
describing the structure of the file. This is typically most useful for
validation, however it **MUST** be acceptable for it to be missing, and
this specification should be seen as the primary definition of structure
over anything defined in the linked schema.

The JSON schema provided at by this element may give details about any
additional fields used that are not defined in this specficiation.

### "metadata" Element {#manifest-metadata}

| Element | Condition | Type | Section | Description |
|---------|-----------|------|---|---|
| title   | Mandatory | String | | The name of the evidence package. |
| authors | Mandatory | Array | (#manifest-metadata-authors) | The authors attributed to this evidence package. |

#### "authors" Array Element {#manifest-metadata-authors}

| Element | Condition | Type | Description |
|---------|-----------|------|---|
| name    | Mandatory | String | The author's name. |
| email   | Optional  | String/Null | The author's email address, although format is not verified. |

### "custom_metadata" Element {#manifest-custom-metadata}

Elements within this object will become custom metadata properties for
test cases in this package. Each object **MUST** have the following
fields:

| Element     | Condition | Type | Section | Description |
|-------------|-----------|------|---|---|
| name        | Mandatory | String | | The name of this custom metadata field. |
| description | Mandatory | String | (#manifest-metadata-authors) | The description of this custom metadata field. |
| primary     | Mandatory | Boolean | (#manifest-custom-metadata-primary) | Is this custom field primary? |

#### "primary" Boolean {#manifest-custom-metadata-primary}

The "primary" value of custom metadata fields **MAY** be false for all
fields, or **MAY** be true for exactly one field. It **MUST NOT** be
true for more than one field.

The purpose of primary is not enforced as part of this specification,
however it should be seen as suggesting that one custom metadata field
is more useful than others, and as such may be used to influence the
information displayed to users, for example an implementor might choose
to show the primary custom metadata value for each test case alongside
it.

### "media" Array Element {#manifest-media}

| Element         | Condition | Type | Description |
|-----------------|-----------|------|---|
| sha256_checksum | Mandatory | String | The SHA256 checksum of the associated media file. |
| mime_type       | Mandatory | String | The Internet Media Type [@!RFC2046] of the associated media file. |

### "test_cases" Array Element {#manifest-test-cases}

| Element    | Condition | Type | Section | Description |
|------------|-----------|------|---------|---|
| id         | Mandatory | String | | The UUID of the test case. If present here, there **MUST** be an associated test case file in the "test_cases" directory of the package with the name "<UUID>.json". |
| attestations | Mandatory | Array of Strings | (#manifest-test-case-attestations) | An array of attestations over this test case. |

#### "attestations" Array {#manifest-test-case-attestations}

The elements within the "attestations" array **MUST** be JWS [@!RFC7515]
signatures. The signature payload must be a SHA256 checksum of a copy of
the test case manifest (i.e. the file "uuid.json"), having been
processed into JSON canonical format as defined in [@!RFC8785].

In environments where it is desirable to frequently validate
attestations, it is recommended to include the "x5c" X.509 certificate,
and a "jku" (JWT Key URL). An implementing client is **RECOMMENDED** to
display the URL in some fashion if the key passes validation as a way to
prove the signing party, however another approach may also be desirable
depending on environment.

Implementing clients **SHOULD NOT** use symmetric key types (although it
may be acceptable for tools that are only used within a limited scope),
and APIs implementing this specification **MAY** choose to be
imcompatible with symmetric types.

As a worked example, a test case may start off like this:

~~~json
{
  "$schema": "https://evidenceangel-schemas.hpkns.uk/testcase.2.schema.json",
  "metadata": {
    "title": "Test Case",
    "execution_datetime": "2025-12-05T20:40:22.743821295Z",
    "passed": null
  },
  "evidence": [
    {
      "kind": "text/plain",
      "value": "plain:Hello, world!"
    }
  ]
}
~~~

This should then be canonicalised:

~~~json
{"$schema":"https://evidenceangel-schemas.hpkns.uk/testcase.2.schema.json","evidence":[{"kind":"text/plain","value":"plain:Hello, world!"}],"metadata":{"execution_datetime":"2025-12-05T20:40:22.743821295Z","passed":null,"title":"Test Case"}}
~~~

A SHA256 checksum can be generated:

~~~text
6cd9684d866d5dacd85064f20d0b3fd423e30946c6b99d1f2defae529360becc
~~~

This can now be signed and the original manifest can be modified:

~~~json
{
  "id": "7928de11-8de8-4bfe-b5b7-cbf07c7066d9",
  "attestations": [
    "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.YmI5YzdkMjczYWY2NzE5NWM1MWM1N2YyNzRjMDc5NTViODZiMDA3MWE0MDU3MWFjOTIwYzE2M2UzNDQxYzUwZQ.KtbRLfAh8UmSxSWYnObpydXyjGO_IPF2acU_x-eFY6dLDBD809zJm6HaTE9jjsQlnX8eGWRIOzKXTWMdgp-fXg"
  ],
  "some_other_value": "Added from somewhere other than this specification!"
}
~~~

## "test_cases" Directory

The test cases directory stores the manifests for each test case within
this evidence package.

Each test case is stored as a JSON file, with a UUIDv4 name [@!RFC9562].
A test case present here **MUST** have a valid entry in the manifest
"test_cases" array defined in (#manifest-test-cases).

### "<uuid>.json" File

| Element  | Condition | Type | Section | Description |
|----------|-----------|------|---|---|
| $schema  | Optional  | String | (#test-case-schema) | The $schema element **MAY** point to a copy of the schema for the manifest. |
| metadata | Mandatory | Object | (#test-case-metadata) | The metadata relating to this test case. |
| evidence | Mandatory | Array | (#test-case-evidence) | The evidence within this test case. |

See an example <uuid>.json file in (#example-test-case).

#### "$schema" Element {#test-case-schema}

This element **MAY** optionally be provided to point to a JSON schema
describing the structure of the file. This is typically most useful for
validation, however it **MUST** be acceptable for it to be missing, and
this specification should be seen as the primary definition of structure
over anything defined in the linked schema.

The JSON schema provided at by this element may give details about any
additional fields used that are not defined in this specficiation.

#### "metadata" Element {#test-case-metadata}

| Element            | Condition | Type | Description |
|--------------------|-----------|------|---|
| title              | Mandatory | String | The title of the test case. |
| execution_datetime | Mandatory | String | The ISO8601 date and time of the execution of this test case starting. |
| passed             | Optional | Enumerated | The state of the test case, if present **MUST** be either the string "pass" or "fail", or null. If absent, it **MUST** be interpreted as null. |
| custom             | Mandatory | Object | Custom metadata values. |

The "custom" field is used to add custom metadata that has been
specified in the package manifest's "custom_metadata" field.
If a value is specified in "custom", it **MUST** be present in the
package manifest, but all values in the package manifest do not need to
be present here. All values **MUST** be strings and are stored as a
simple key-value map, with the custom field ID defined in the manifest
as the key.

#### "evidence" Array Element {#test-case-evidence}

| Element           | Condition | Type | Section | Description |
|-------------------|-----------|------|---|---|
| kind              | Mandatory | String | (#evidence-kind) | The Internet Media Type [@!RFC2046] of data stored. |
| value             | Mandatory | String | (#evidence-value) | The data stored within this piece of evidence. |
| caption           | Optional  | String/Null | | An optional caption for this piece of evidence. |
| original_filename | Optional  | String/Null | | The original filename. |

##### "kind" {#evidence-kind}

The "kind" of evidence **MUST** be an Internet Media Type [@!RFC2046].

For more information about each type, see (#kinds-of-evidence).

##### "value" {#evidence-value}

The "value" **MUST** be one of the following acceptable patterns:

* "plain:" followed by plain text;
* "media:" followed by a media file SHA256 hash, or;
* "base64:" followed by a base64 string of data without padding.

## "media" Directory

The "media" directory stores data in files within the ZIP archive that
would be otherwise impractical to store directly in the test cases.

Files stored in this directory are of abitrary type. They **MUST** be
named by their SHA256 checksum [@!RFC6234] with no extension. Their
SHA256 checksum and media type **MUST** be stored in the package
manifest "media" element.

In the unlikely event that there is a checksum clash, there is currently
no preferred method for resolving this. The probability of such a
situation is decided to be acceptably low given the expected size and
number of files stored in an evidence package, however implementors
**MAY** choose to store the clashing file as base64 data instead of as
an additional media file.

# Handling an Evidence Package

## Locking

When loading an evidence package, implemetors **MUST** use a lock file
with the file name ".~lock." followed by the full name of the package it
protects, followed by "#", for example for a package called
"example.evp", the lock file **MUST** be called ".~lock.example.evp#".
It **MUST** be located adjacent (in the same directory as) the evidence
package. The file **MUST** contain the process ID of the process holding
the lock.

The lock file should be considered as locking the package if it is
present, regardless of contents.

If either of these is not the case, it should be assumed that the there
is no current lock over the package.

## Media Loading

Software implementing the evidence package format **MUST NOT** load
files from the "media" directory into memory until it is needed for
display or for extraction. Implementors **MUST** use streams to load
media files to avoid trying to load the entire file into memory as it
may not fit.

# Kinds of Evidence {#kinds-of-evidence}

Evidence packages can support any valid Internet Media Type [@!RFC2046]
as evidence. Implementors of this specification **MUST** be able to
display the following types:

| Media Type               | Description                                      |
|--------------------------|--------------------------------------------------|
| text/plain               | Plain text with no formatting.                   |
| text/markdown            | Text with markdown support.                      |
| text/vnd.angel.http-data | An HTTP request/response pair.                   |
| image/*                  | An image that should be rendered where possible. |

Common image formats **SHOULD** be rendered where possible, but it is
not required to support every possible type of image.

Markdown **SHOULD** be rendered where possible, but it may be adapted
for security reasons. If it is changed before display, a notice **MUST**
be displayed to the user disclosing that it has been adjusted for
security. For example, it is acceptable to strip raw HTML tags before
rendering.

Other media types **MUST** be supported insofar as being able to extract
the data from the evidence package so that they can be opened in other
software.

## HTTP Requests {#http-requests}

Where text/vnd.angel.http-data is used, an HTTP request and
response **MUST** be present in plain text, and a Record Separator
character (0x1e) **MUST** be used to split the request and response
portion. In other words, the format **MUST** comply with the following
regular expression:

~~~regex
^(?<request>[.\r\n]*)\x1e(?<response>[.\r\n]*)$
~~~

For example the separator is present at <<1>>:

~~~http
GET / HTTP/1.1
Host: example.com
User-Agent: HTTPie

\x1eHTTP/1.1 200 OK //<<1>>
Cache-Control: max-age=1366
Connection: close
...
~~~

# Extending Behaviours of an Evidence Package

Every JSON file within an evidence package **MAY** have new fields
added, and as such extended behaviours **MAY** be implemented, however
implementors **MUST** be able to load an evidence package without these
additional fields.

When an implementor loads a file with fields it cannot understand, it
**MUST** retain the fields on saving the file.

# IANA Considerations

This document acts as the specification for the media type
application/vnd.angel.evidence-package. Additionally, the media type
text/vnd.angel.http-data is defined in (#http-requests).

# Security Considerations

The evidence package format can store arbitrary files that may or may
not be executable. Implementors **MUST NOT** execute any file contained
within and **SHALL** only extract the contained files if needed.

Otherwise, there are no concerns for security from the file type itself.

{backmatter}

# Example Archive Layout {#example-archive}

~~~
example.evp
 |- manifest.json
 |- media
 |   \- 203073da0b36a5921f2914e2093abcae7eb987846f405b438c25792bab1617fa
 \- test_cases
     \- eabb5d31-a958-4609-ac98-83365e14d18b.json
~~~

# Example Package Manifest JSON {#example-manifest}

~~~json
{
  "metadata": {
    "title": "Example Evidence Package",
    "authors": [
      {
        "name": "Anonymous Author"
      },
      {
        "name": "Lily Hopkins",
        "email": "lily@hpkns.uk"
      }
    ]
  },
  "custom_metadata": {
    "example": {
      "name": "Example Metadata Field",
      "description": "A field showing that custom fields can be added",
      "primary": true
    }
  },
  "media": [
    {
      "sha256_checksum": "203073da0b36a5921f2914e2093abcae7eb987846f405b438c25792bab1617fa",
      "mime_type": "text/plain"
    }
  ],
  "test_cases": [
    {
      "id": "eabb5d31-a958-4609-ac98-83365e14d18b",
      "attestations": [
        "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.YmI5YzdkMjczYWY2NzE5NWM1MWM1N2YyNzRjMDc5NTViODZiMDA3MWE0MDU3MWFjOTIwYzE2M2UzNDQxYzUwZQ.KtbRLfAh8UmSxSWYnObpydXyjGO_IPF2acU_x-eFY6dLDBD809zJm6HaTE9jjsQlnX8eGWRIOzKXTWMdgp-fXg"
      ]
    }
  ]
}
~~~

# Example Test Case Manifest JSON {#example-test-case}

~~~json
{
  "metadata": {
    "title": "Example Test Case",
    "execution_datetime": "2025-05-01T11:13:29+01:00",
    "passed": null,
    "custom": {
      "example": "Example custom metadata field value"
    }
  },
  "evidence": [
    {
      "kind":"text/plain",
      "value":"plain:This is some text based evidence"
    },
    {
      "kind":"text/plain",
      "value":"base64:VGhpcyBpcyBzb21lIHRleHQgYmFzZWQgYmFzZTY0IGVuY29kZWQgZXZpZGVuY2U"
    },
    {
      "kind":"text/plain",
      "value":"media:203073da0b36a5921f2914e2093abcae7eb987846f405b438c25792bab1617fa",
      "caption": "An example file",
      "original_filename": "example.txt"
    },
    {
      "kind":"image/png",
      "value":"media:c561967275f002e65b222b4577378f5a20a5881edd00fbe648beef6b4f4971a9",
      "caption": "An example image",
      "original_filename": "image.png"
    }
  ]
}
~~~

# JSON Schema for Package Manifest

<{{manifest.2.schema.json}}

# JSON Schema for Test Case Manifest

<{{testcase.2.schema.json}}
