// Integration tests for MTGJSON API compatibility
use mtgjson_rust::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_python_methods() {
        let mut card1 = MtgjsonCardObject::new(false);
        card1.name = "Lightning Bolt".to_string();
        card1.set_code = "LEA".to_string();
        card1.number = "161".to_string();
        card1.uuid = "12345-67890".to_string();

        let mut card2 = MtgjsonCardObject::new(false);
        card2.name = "Counterspell".to_string();
        card2.set_code = "LEA".to_string();
        card2.number = "55".to_string();
        card2.uuid = "98765-43210".to_string();

        // Test __eq__ method
        assert!(!card1.__eq__(&card2));
        assert!(card1.__eq__(&card1));

        // Test __str__ method
        let str_repr = card1.__str__();
        assert_eq!(str_repr, "Lightning Bolt (LEA) #161");

        // Test __repr__ method
        let repr_str = card1.__repr__();
        assert_eq!(repr_str, "MtgjsonCardObject(name='Lightning Bolt', set_code='LEA', uuid='12345-67890')");

        // Test __hash__ method
        let hash_val = card1.__hash__();
        assert!(hash_val > 0);

        // Test __lt__ method for sorting
        // Counterspell should come before Lightning Bolt alphabetically
        assert!(card2.__lt__(&card1));
        assert!(!card1.__lt__(&card2));
    }

    #[test]
    fn test_set_builder_functions() {
        // Test parse_card_types
        let (super_types, types, sub_types) = parse_card_types("Legendary Creature — Human Wizard");
        assert_eq!(super_types, vec!["Legendary"]);
        assert_eq!(types, vec!["Creature"]);
        assert_eq!(sub_types, vec!["Human", "Wizard"]);

        // Test get_card_colors
        let colors = get_card_colors("{2}{W}{U}");
        assert_eq!(colors, vec!["W", "U"]);

        // Test get_card_cmc
        assert_eq!(get_card_cmc("{3}"), 3.0);
        assert_eq!(get_card_cmc("{2}{W}{U}"), 4.0);

        // Test is_number
        assert!(is_number("123"));
        assert!(is_number("12.5"));
        assert!(!is_number("abc"));
        assert!(!is_number("X"));
    }

    #[test]
    fn test_card_type_field_access() {
        let mut card = MtgjsonCardObject::new(false);
        
        // Test that the type field is accessible despite being type_ internally
        card.type_ = "Creature — Human".to_string();
        assert_eq!(card.type_, "Creature — Human");
        
        // In Python, this would be accessed as card.type
        // The #[pyo3(name = "type")] should handle this mapping
    }

    #[test]
    fn test_high_performance_modules() {
        // Test that high-performance modules are accessible
        let output_gen = OutputGenerator::new("./test_output".to_string(), true);
        assert_eq!(output_gen.output_path, "./test_output");
        assert!(output_gen.pretty_print);

        let price_builder = PriceBuilder::new();
        assert!(price_builder.providers.contains(&"TCGPlayer".to_string()));
        assert_eq!(price_builder.archive_days, 30);

        let parallel_processor = ParallelProcessor::new(Some(16));
        assert_eq!(parallel_processor.pool_size, 16);

        let parallel_iterator = ParallelIterator::new(Some(500), Some(8));
        assert_eq!(parallel_iterator.chunk_size, 500);
        assert_eq!(parallel_iterator.pool_size, 8);
    }

    #[test]
    fn test_backwards_compatibility() {
        let card1 = MtgjsonCardObject::new(false);
        let card2 = MtgjsonCardObject::new(false);

        // Test that legacy methods still work but are deprecated
        #[allow(deprecated)]
        {
            assert!(card1.eq(&card2)); // Should work but be deprecated
            
            let comparison_result = card1.compare(&card2);
            assert!(comparison_result.is_ok());
            assert_eq!(comparison_result.unwrap(), 0); // Should be equal
        }
    }

    #[test]
    fn test_json_object_trait() {
        let card = MtgjsonCardObject::new(false);
        
        // Test that JsonObject trait is implemented
        let keys_to_skip = card.build_keys_to_skip();
        assert!(keys_to_skip.contains("is_token"));
        assert!(keys_to_skip.contains("raw_purchase_urls"));
        assert!(keys_to_skip.contains("set_code"));
    }
}