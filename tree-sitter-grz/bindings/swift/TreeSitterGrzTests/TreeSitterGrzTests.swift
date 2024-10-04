import XCTest
import SwiftTreeSitter
import TreeSitterGrz

final class TreeSitterGrzTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_grz())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Grz grammar")
    }
}
