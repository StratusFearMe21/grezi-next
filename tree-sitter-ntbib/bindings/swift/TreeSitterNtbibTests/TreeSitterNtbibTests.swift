import XCTest
import SwiftTreeSitter
import TreeSitterNtbib

final class TreeSitterNtbibTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_ntbib())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Ntbib grammar")
    }
}
