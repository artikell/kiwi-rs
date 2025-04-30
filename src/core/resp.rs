//  Copyright (c) 2017-present, arana-db Community.  All rights reserved.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

pub fn parse_resp_message(_input: &[u8]) -> Result<Vec<String>, String> {
    let result = Vec::new();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resp2() {
        let input = b"*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n";
        let result = parse_resp_message(input).unwrap();
        assert_eq!(result, Vec::<String>::new());
    }
}